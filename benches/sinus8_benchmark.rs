use criterion::{criterion_group, criterion_main, Criterion};
use sinus8;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sinus8 8 Bytes", |b| b.iter(|| sinus8::sinus8("ABCDEFGH")));

    let ascii_data = (33..127).map(|c| c as u8 as char).collect::<String>();
    c.bench_function("sinus8 readable ascii", |b| b.iter(|| sinus8::sinus8(&ascii_data)));

    let large_data = (0..10_000_000).map(|n| {
        let mut ascii_code = (n % 26) as u8;
        if n % 3 == 1 {
            ascii_code += 65;
        } else {
            ascii_code += 97;
        }
        ascii_code as char
    }).collect::<String>();
    c.bench_function("sinus8 large data", |b| b.iter(|| sinus8::sinus8(&large_data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
