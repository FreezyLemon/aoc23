use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc23::*;

macro_rules! bench_days {
    () => {};
    ($d: literal, $($rest:tt)*) => {
        bench_for_day!($d, 1);
        bench_for_day!($d, 2);
        bench_days!($($rest)*);
    };
}

macro_rules! bench_for_day {
    ($d:literal, $p:literal) => {
        paste::paste! {
            fn [<bench_day $d _part $p>](c: &mut Criterion) {
                c.bench_function(&format!("day {} part {}", $d, $p), |b| {
                    let day = [<Day $d Part $p>];
                    let input = get_input(&format!("d{}p{}", $d, $p), None).unwrap();

                    b.iter(|| day.solve(black_box(&input)));
                });
            }
        }
    };
}

macro_rules! bench_group {
    ($($d:literal, )+) => {
        paste::paste! {
            criterion_group!(
                benches,
                $(
                    [<bench_day $d _part 1>],
                    [<bench_day $d _part 2>],
                )+
            );
        }
    };
}

bench_days!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19,);
bench_group!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19,);

fn bench_day1_part2_malox(c: &mut Criterion) {
    c.bench_function("day 1 part 2 malox", |b| {
        let day = Day1Part2Malox;
        let input = get_input("d1p2", None).unwrap();

        b.iter(|| day.solve(black_box(&input)));
    });
}

criterion_group!(rest, bench_day1_part2_malox);

criterion_main!(benches, rest);
