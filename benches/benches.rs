use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc23::*;

fn bench_day1_part2(c: &mut Criterion) {
    c.bench_function("day 1 part 2", |b| {
        let day = Day1Part2;
        let input = get_input("day01-part2", None).unwrap();

        b.iter(|| day.solve(black_box(input.clone())));
    });
}

fn bench_day1_part2_malox(c: &mut Criterion) {
    c.bench_function("day 1 part 2 malox", |b| {
        let day = Day1Part2Malox;
        let input = get_input("day01-part2", None).unwrap();

        b.iter(|| day.solve(black_box(input.clone())));
    });
}

criterion_group!(
    benches,
    bench_day1_part2,
    bench_day1_part2_malox,
);

criterion_main!(benches);

