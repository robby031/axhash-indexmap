use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use indexmap::IndexMap as StdIndexMap;

use axhash_indexmap::{AxIndexMap, map_with_capacity};

const N: usize = 100_000;
const SEED: u64 = 0xabcd_ef01_2345_6789;

fn configured_criterion() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .warm_up_time(Duration::from_secs(2))
        .sample_size(100)
}

struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

fn u64_keys(n: usize, seed: u64) -> Vec<u64> {
    let mut rng = SplitMix64(seed);
    (0..n).map(|_| rng.next()).collect()
}

fn string_keys(n: usize, seed: u64) -> Vec<String> {
    let mut rng = SplitMix64(seed);
    (0..n)
        .map(|i| format!("key-{:010}-{:016x}", i, rng.next()))
        .collect()
}

fn bench_insert(c: &mut Criterion) {
    {
        let keys = u64_keys(N, SEED);
        let mut group = c.benchmark_group("insert/u64");
        group.throughput(Throughput::Elements(N as u64));

        group.bench_function(BenchmarkId::new("AxIndexMap", N), |b| {
            b.iter(|| {
                let mut map = map_with_capacity::<u64, u64>(N);
                for &k in &keys {
                    map.insert(k, black_box(k.wrapping_mul(6364136223846793005)));
                }
                black_box(map)
            });
        });

        group.bench_function(BenchmarkId::new("IndexMap (SipHash)", N), |b| {
            b.iter(|| {
                let mut map = StdIndexMap::with_capacity(N);
                for &k in &keys {
                    map.insert(k, black_box(k.wrapping_mul(6364136223846793005)));
                }
                black_box(map)
            });
        });

        group.finish();
    }

    {
        let keys = string_keys(N, SEED);
        let mut group = c.benchmark_group("insert/string");
        group.throughput(Throughput::Elements(N as u64));

        group.bench_function(BenchmarkId::new("AxIndexMap", N), |b| {
            b.iter(|| {
                let mut map = map_with_capacity::<&str, usize>(N);
                for (i, k) in keys.iter().enumerate() {
                    map.insert(k.as_str(), black_box(i));
                }
                black_box(map)
            });
        });

        group.bench_function(BenchmarkId::new("IndexMap (SipHash)", N), |b| {
            b.iter(|| {
                let mut map = StdIndexMap::with_capacity(N);
                for (i, k) in keys.iter().enumerate() {
                    map.insert(k.as_str(), black_box(i));
                }
                black_box(map)
            });
        });

        group.finish();
    }
}

fn bench_lookup(c: &mut Criterion) {
    let insert_keys = u64_keys(N, SEED);
    let absent_keys = u64_keys(N, SEED ^ 0xffff_ffff_ffff_ffff);

    let mixed_keys: Vec<u64> = insert_keys
        .iter()
        .zip(absent_keys.iter())
        .flat_map(|(&h, &m)| [h, m])
        .collect();

    let ax_map: AxIndexMap<u64, u64> = insert_keys.iter().map(|&k| (k, k)).collect();
    let std_map: StdIndexMap<u64, u64> = insert_keys.iter().map(|&k| (k, k)).collect();

    {
        let mut group = c.benchmark_group("lookup/hit");
        group.throughput(Throughput::Elements(N as u64));

        group.bench_function(BenchmarkId::new("AxIndexMap", N), |b| {
            b.iter(|| {
                let mut sum = 0u64;
                for k in &insert_keys {
                    if let Some(&v) = ax_map.get(k) {
                        sum = sum.wrapping_add(v);
                    }
                }
                black_box(sum)
            });
        });

        group.bench_function(BenchmarkId::new("IndexMap (SipHash)", N), |b| {
            b.iter(|| {
                let mut sum = 0u64;
                for k in &insert_keys {
                    if let Some(&v) = std_map.get(k) {
                        sum = sum.wrapping_add(v);
                    }
                }
                black_box(sum)
            });
        });

        group.finish();
    }

    {
        let mut group = c.benchmark_group("lookup/mixed");
        group.throughput(Throughput::Elements(mixed_keys.len() as u64));

        group.bench_function(BenchmarkId::new("AxIndexMap", N), |b| {
            b.iter(|| {
                let mut sum = 0u64;
                for k in &mixed_keys {
                    if let Some(&v) = ax_map.get(k) {
                        sum = sum.wrapping_add(v);
                    }
                }
                black_box(sum)
            });
        });

        group.bench_function(BenchmarkId::new("IndexMap (SipHash)", N), |b| {
            b.iter(|| {
                let mut sum = 0u64;
                for k in &mixed_keys {
                    if let Some(&v) = std_map.get(k) {
                        sum = sum.wrapping_add(v);
                    }
                }
                black_box(sum)
            });
        });

        group.finish();
    }
}

fn bench_iter(c: &mut Criterion) {
    let insert_keys = u64_keys(N, SEED);

    let ax_map: AxIndexMap<u64, u64> = insert_keys.iter().map(|&k| (k, k)).collect();
    let std_map: StdIndexMap<u64, u64> = insert_keys.iter().map(|&k| (k, k)).collect();

    let mut group = c.benchmark_group("iteration");
    group.throughput(Throughput::Elements(N as u64));

    group.bench_function(BenchmarkId::new("AxIndexMap", N), |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for (_, &v) in &ax_map {
                sum = sum.wrapping_add(v);
            }
            black_box(sum)
        });
    });

    group.bench_function(BenchmarkId::new("IndexMap (SipHash)", N), |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for (_, &v) in &std_map {
                sum = sum.wrapping_add(v);
            }
            black_box(sum)
        });
    });

    group.finish();
}

fn bench_index_access(c: &mut Criterion) {
    let insert_keys = u64_keys(N, SEED);
    // Prepare a shuffled index list so we don't just iterate sequentially.
    let mut rng = SplitMix64(SEED ^ 0xdead);
    let indices: Vec<usize> = (0..N).map(|_| (rng.next() as usize) % N).collect();

    let ax_map: AxIndexMap<u64, u64> = insert_keys.iter().map(|&k| (k, k)).collect();
    let std_map: StdIndexMap<u64, u64> = insert_keys.iter().map(|&k| (k, k)).collect();

    let mut group = c.benchmark_group("index_access");
    group.throughput(Throughput::Elements(N as u64));

    group.bench_function(BenchmarkId::new("AxIndexMap", N), |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for &i in &indices {
                if let Some((_, &v)) = ax_map.get_index(i) {
                    sum = sum.wrapping_add(v);
                }
            }
            black_box(sum)
        });
    });

    group.bench_function(BenchmarkId::new("IndexMap (SipHash)", N), |b| {
        b.iter(|| {
            let mut sum = 0u64;
            for &i in &indices {
                if let Some((_, &v)) = std_map.get_index(i) {
                    sum = sum.wrapping_add(v);
                }
            }
            black_box(sum)
        });
    });

    group.finish();
}

criterion_group!(
    name    = benches;
    config  = configured_criterion();
    targets = bench_insert, bench_lookup, bench_iter, bench_index_access
);
criterion_main!(benches);
