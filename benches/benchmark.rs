use advent_of_code_2019_rust;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::io;
use std::path::Path;

fn day_01(c: &mut Criterion) {
    let file = fs::read_to_string(Path::new(".").join("inputs").join("day_01.txt")).unwrap();
    c.bench_function("day_01_part_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day01::part01(reader).unwrap();
        })
    });

    c.bench_function("day_01_part_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day01::part02(reader).unwrap();
        })
    });
}
fn day_02(c: &mut Criterion) {
    let file = fs::read_to_string(Path::new(".").join("inputs").join("day_02.txt")).unwrap();
    c.bench_function("day_02_part_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day02::part01(reader, 12, 2).unwrap();
        })
    });

    c.bench_function("day_02_part_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day02::part02(reader, 19690720).unwrap();
        })
    });
}
fn day_03(c: &mut Criterion) {
    let file = fs::read_to_string(Path::new(".").join("inputs").join("day_03.txt")).unwrap();
    c.bench_function("day_03_part_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day03::part01(reader).unwrap();
        })
    });

    c.bench_function("day_03_part_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day03::part02(reader).unwrap();
        })
    });
}

fn day_04(c: &mut Criterion) {
    let file = fs::read_to_string(Path::new(".").join("inputs").join("day_04.txt")).unwrap();
    c.bench_function("day_04_part_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day04::part01(reader).unwrap();
        })
    });

    c.bench_function("day_04_part_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day04::part02(reader).unwrap();
        })
    });
}

fn day_05(c: &mut Criterion) {
    let file = fs::read_to_string(Path::new(".").join("inputs").join("day_05.txt")).unwrap();
    c.bench_function("day_05_part_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day05::part01(reader, 1).unwrap();
        })
    });

    c.bench_function("day_05_part_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day05::part02(reader, 5).unwrap();
        })
    });
}

fn day_06(c: &mut Criterion) {
    let file = fs::read_to_string(Path::new(".").join("inputs").join("day_06.txt")).unwrap();
    c.bench_function("day_06_part_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day06::part01(reader).unwrap();
        })
    });

    c.bench_function("day_06_part_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day06::part02(
                reader,
                "YOU".to_string(),
                "SAN".to_string(),
            )
            .unwrap();
        })
    });
}

fn day_07(c: &mut Criterion) {
    let file = fs::read_to_string(Path::new(".").join("inputs").join("day_07.txt")).unwrap();
    c.bench_function("day_07_part_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day07::part01(reader).unwrap();
        })
    });

    c.bench_function("day_07_part_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(file.as_bytes());
            advent_of_code_2019_rust::days::day07::part02(reader).unwrap();
        })
    });
}

criterion_group!(
    name= benches;
    config = Criterion::default();
    targets = day_01, day_02, day_03, day_04, day_05, day_06, day_07
);
criterion_main!(benches);
