use criterion::{criterion_group, criterion_main, Criterion};
mod key_value;

fn all_benches(c: &mut Criterion) {
    env_logger::init();
    key_value::benches(c);
}

criterion_group!(benches, all_benches);
criterion_main!(benches);
