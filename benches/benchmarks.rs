use std::fs;
use std::io;
use std::time::Duration;

use aoc2019;
use criterion::{criterion_group, criterion_main, Criterion};

fn target_01(c: &mut Criterion) {
    let day01 = fs::read_to_string("data/day1").unwrap();
    c.bench_function("day_01", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day01.as_bytes());
            aoc2019::day01::run(reader).unwrap();
        })
    });
}

fn target_02(c: &mut Criterion) {
    let day02 = fs::read_to_string("data/day2").unwrap();
    c.bench_function("day_02", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day02.as_bytes());
            aoc2019::day02::run(reader).unwrap();
        })
    });
}

fn target_03(c: &mut Criterion) {
    let day03 = fs::read_to_string("data/day3").unwrap();
    c.bench_function("day_03", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day03.as_bytes());
            aoc2019::day03::run(reader).unwrap();
        })
    });
}

fn target_04(c: &mut Criterion) {
    let day04 = fs::read_to_string("data/day4").unwrap();
    c.bench_function("day_04", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day04.as_bytes());
            aoc2019::day04::run(reader).unwrap();
        })
    });
}

fn target_05(c: &mut Criterion) {
    let day05 = fs::read_to_string("data/day5").unwrap();
    c.bench_function("day_05", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day05.as_bytes());
            aoc2019::day05::run(reader).unwrap();
        })
    });
}

fn target_06(c: &mut Criterion) {
    let day06 = fs::read_to_string("data/day6").unwrap();
    c.bench_function("day_06", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day06.as_bytes());
            aoc2019::day06::run(reader).unwrap();
        })
    });
}

fn target_07(c: &mut Criterion) {
    let day07 = fs::read_to_string("data/day7").unwrap();
    c.bench_function("day_07", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day07.as_bytes());
            aoc2019::day07::run(reader).unwrap();
        })
    });
}

fn target_08_v2(c: &mut Criterion) {
    let day08 = fs::read_to_string("data/day8").unwrap();
    c.bench_function("day_08_v2", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day08.as_bytes());
            aoc2019::day08_v2::run(reader).unwrap();
        })
    });
}

fn target_08(c: &mut Criterion) {
    let day08 = fs::read_to_string("data/day8").unwrap();
    c.bench_function("day_08", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(day08.as_bytes());
            aoc2019::day08::run(reader).unwrap();
        })
    });
}

fn target_09(c: &mut Criterion) {
    let data = fs::read_to_string("data/day9").unwrap();
    c.bench_function("day_09", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(data.as_bytes());
            aoc2019::day09::run(reader).unwrap();
        })
    });
}

fn target_10(c: &mut Criterion) {
    let data = fs::read_to_string("data/day10").unwrap();
    c.bench_function("day_10", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(data.as_bytes());
            aoc2019::day10::run(reader).unwrap();
        })
    });
}

fn target_12(c: &mut Criterion) {
    let data = fs::read_to_string("data/day12").unwrap();
    c.bench_function("day_12", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(data.as_bytes());
            aoc2019::day12::run(reader).unwrap();
        })
    });
}

fn target_14(c: &mut Criterion) {
    let data = fs::read_to_string("data/day14").unwrap();
    c.bench_function("day_14", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(data.as_bytes());
            aoc2019::day14::run(reader).unwrap();
        })
    });
}

fn target_16(c: &mut Criterion) {
    let data = fs::read_to_string("data/day16").unwrap();
    c.bench_function("day_16", |b| {
        b.iter(|| {
            let reader = io::BufReader::new(data.as_bytes());
            aoc2019::day16::run(reader).unwrap();
        })
    });
}

criterion_group! {
    name = group;
    config = Criterion::default().warm_up_time(Duration::from_secs(5));
    targets = target_01, target_02, target_03, target_04, target_05, target_06, target_07, target_08, target_08_v2, target_09, target_10, target_12, target_14, target_16
}

criterion_main!(group);
