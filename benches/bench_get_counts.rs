#[cfg(feature = "bench")]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[cfg(feature = "bench")]
use esaxx_rs::{get_counts_cpp, sais::get_counts};

#[cfg(feature = "bench")]
fn get_groups_buckets(c: &mut Criterion) {
    let string: Vec<u32> = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.".chars().map(|c| c as u32).collect();

    let k: u32 = 256;

    let mut counts = vec![0; k as usize];
    let n = string.len() as u32;

    c.bench_function("get_counts_cpp", |b| {
        b.iter(|| get_counts_cpp(&string, &mut counts, n, k))
    });
    c.bench_function("get_counts_rs", |b| {
        b.iter(|| get_counts(black_box(&string), &mut counts, n, k))
    });
}

#[cfg(feature = "bench")]
criterion_group!(benches, get_groups_buckets);
#[cfg(feature = "bench")]
criterion_main!(benches);
#[cfg(not(feature = "bench"))]
fn main() {}
