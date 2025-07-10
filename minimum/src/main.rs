#![feature(test)]

extern crate test;

use test::Bencher;

fn find_min_iter_ref(a: &[i32]) -> i32 {
    *a.iter().min().unwrap()
}

fn find_min_iter(a: &[i32]) -> i32 {
    a.iter().copied().min().unwrap()
}

fn find_min_for_loop(a: &[i32]) -> i32 {
    let mut min = a[0];
    for &x in a.iter() {
        if x < min {
            min = x
        }
    }
    min
}

fn find_min_iter_avx2(a: &[i32]) -> i32 {
    #[target_feature(enable = "avx2")]
    unsafe fn run(a: &[i32]) -> i32 {
        let mut min = a[0];
        for &x in a.iter() {
            if x < min {
                min = x
            }
        }
        min
    }
    unsafe { run(a) }
}

fn main() {
    println!("Hello, world!");
}

fn generate_data() -> Vec<i32> {
    (0..100_000).rev().collect()
}

#[bench]
fn bench_iter_ref(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| find_min_iter_ref(&data));
}

#[bench]
fn bench_iter(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| find_min_iter(&data));
}

#[bench]
fn bench_for_loop(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| find_min_for_loop(&data));
}

#[bench]
fn bench_iter_avx2(b: &mut Bencher) {
    let data = generate_data();
    b.iter(|| find_min_iter_avx2(&data));
}
