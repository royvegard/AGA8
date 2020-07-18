extern crate criterion;

use aga8::Detail;
use aga8::Gerg2008;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_detail_setup(c: &mut Criterion) {
    let mut aga8_test: Detail = Detail::new();

    c.bench_function("Detail_setup", |b| {
        b.iter(|| {
            aga8_test.setup();
        })
    });
}

fn bench_detail_density(c: &mut Criterion) {
    let mut aga8_test: Detail = Detail::new();
    aga8_test.setup();
    aga8_test.x = [
        0.778_240, 0.020_000, 0.060_000, 0.080_000, 0.030_000, 0.001_500, 0.003_000, 0.000_500,
        0.001_650, 0.002_150, 0.000_880, 0.000_240, 0.000_150, 0.000_090, 0.004_000, 0.005_000,
        0.002_000, 0.000_100, 0.002_500, 0.007_000, 0.001_000,
    ];
    aga8_test.p = 50_000.0;
    aga8_test.t = 400.0;

    c.bench_function("Detail_density", |b| {
        b.iter(|| {
            aga8_test.density_detail();
        })
    });
}

fn bench_detail_properties(c: &mut Criterion) {
    let mut aga8_test: Detail = Detail::new();
    aga8_test.setup();
    aga8_test.x = [
        0.778_240, 0.020_000, 0.060_000, 0.080_000, 0.030_000, 0.001_500, 0.003_000, 0.000_500,
        0.001_650, 0.002_150, 0.000_880, 0.000_240, 0.000_150, 0.000_090, 0.004_000, 0.005_000,
        0.002_000, 0.000_100, 0.002_500, 0.007_000, 0.001_000,
    ];
    aga8_test.p = 50_000.0;
    aga8_test.t = 400.0;
    aga8_test.density_detail();

    c.bench_function("Detail_properties", |b| {
        b.iter(|| {
            aga8_test.properties_detail();
        })
    });
}

fn bench_gerg_setup(c: &mut Criterion) {
    let mut gerg_test: Gerg2008 = Gerg2008::default();
    gerg_test.x = [
        0.0, 0.77824, 0.02, 0.06, 0.08, 0.03, 0.0015, 0.003, 0.0005, 0.00165, 0.00215, 0.00088,
        0.00024, 0.00015, 0.00009, 0.004, 0.005, 0.002, 0.0001, 0.0025, 0.007, 0.001,
    ];

    c.bench_function("Gerg_setup", |b| {
        b.iter(|| {
            gerg_test.setup();
        })
    });
}

fn bench_gerg_density(c: &mut Criterion) {
    let mut gerg_test: Gerg2008 = Gerg2008::default();
    gerg_test.x = [
        0.0, 0.77824, 0.02, 0.06, 0.08, 0.03, 0.0015, 0.003, 0.0005, 0.00165, 0.00215, 0.00088,
        0.00024, 0.00015, 0.00009, 0.004, 0.005, 0.002, 0.0001, 0.0025, 0.007, 0.001,
    ];
    gerg_test.setup();
    gerg_test.molarmass();
    gerg_test.t = 400.0;
    gerg_test.p = 50000.0;
    gerg_test.d = 6.36570;
    gerg_test.z = 0.0;
    c.bench_function("Gerg_density", |b| {
        b.iter(|| {
            gerg_test.density(0);
        })
    });
}

fn bench_gerg_properties(c: &mut Criterion) {
    let mut gerg_test: Gerg2008 = Gerg2008::default();
    gerg_test.x = [
        0.0, 0.77824, 0.02, 0.06, 0.08, 0.03, 0.0015, 0.003, 0.0005, 0.00165, 0.00215, 0.00088,
        0.00024, 0.00015, 0.00009, 0.004, 0.005, 0.002, 0.0001, 0.0025, 0.007, 0.001,
    ];
    gerg_test.setup();
    gerg_test.molarmass();
    gerg_test.t = 400.0;
    gerg_test.p = 50000.0;
    gerg_test.d = 6.36570;
    gerg_test.z = 0.0;
    gerg_test.density(0);

    c.bench_function("Gerg_properties", |b| {
        b.iter(|| {
            gerg_test.properties();
        })
    });
}

criterion_group!(
    benches,
    bench_detail_setup,
    bench_detail_density,
    bench_detail_properties,
    bench_gerg_setup,
    bench_gerg_density,
    bench_gerg_properties,
);
criterion_main!(benches);
