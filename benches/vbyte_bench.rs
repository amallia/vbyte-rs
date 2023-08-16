use criterion::{criterion_group, criterion_main, Criterion};
use vbyte_rs::{vbyte_decode, vbyte_encode};

fn vbyte_encode_benchmark(c: &mut Criterion) {
    c.bench_function("vbyte_encode", |b| {
        b.iter(|| {
            let mut output = Vec::new();
            for i in 0..100_000 {
                vbyte_encode(i, &mut output);
            }
        });
    });
}

fn vbyte_decode_benchmark(c: &mut Criterion) {
    // First, we'll prepare some encoded data to use in the benchmark
    let mut encoded_data = Vec::new();
    for i in 0..100_000 {
        vbyte_encode(i, &mut encoded_data);
    }
    let chunks: Vec<_> = encoded_data.chunks(5).collect(); // Assuming a maximum of 5 bytes per encoded number

    c.bench_function("vbyte_decode", |b| {
        b.iter(|| {
            for chunk in &chunks {
                vbyte_decode(chunk).unwrap();
            }
        });
    });
}

criterion_group!(benches, vbyte_encode_benchmark, vbyte_decode_benchmark);
criterion_main!(benches);
