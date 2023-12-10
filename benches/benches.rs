use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc23::*;

fn bench_day1_part2(c: &mut Criterion) {
    c.bench_function("day 1 part 2", |b| {
        let day = Day1Part2;
        let input = get_input("d1p2", None).unwrap();

        b.iter(|| day.solve(black_box(input.clone())));
    });
}

fn bench_day1_part2_malox(c: &mut Criterion) {
    c.bench_function("day 1 part 2 malox", |b| {
        let day = Day1Part2Malox;
        let input = get_input("d1p2", None).unwrap();

        b.iter(|| day.solve(black_box(input.clone())));
    });
}

fn bench_day4_part2(c: &mut Criterion) {
    c.bench_function("day 4 part 2", |b| {
        let day = Day4Part2;
        let input = get_input("d4p2", None).unwrap();

        b.iter(|| day.solve(black_box(input.clone())));
    });
}

fn bench_day9_part1(c: &mut Criterion) {
    c.bench_function("day 9 part 1", |b| {
        let day = Day9Part1;
        let input = get_input("d9p1", None).unwrap();

        b.iter(|| day.solve(black_box(input.clone())));
    });
}

fn bench_day9_part2(c: &mut Criterion) {
    c.bench_function("day 9 part 2", |b| {
        let day = Day9Part2;
        let input = get_input("d9p2", None).unwrap();

        b.iter(|| day.solve(black_box(input.clone())));
    });
}

criterion_group!(
    benches,
    bench_day1_part2,
    bench_day1_part2_malox,
    bench_day4_part2,
    bench_day9_part1,
    bench_day9_part2,
);

criterion_main!(benches);

