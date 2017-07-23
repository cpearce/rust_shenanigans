extern crate rand;

use rand::Rng;

fn random_vec() -> Vec<i32> {
    let mut v : Vec<i32> = Vec::new();
    for _ in 0..50 {
        v.push(rand::thread_rng().gen_range(0,1000));
    }
    v
}

fn is_sorted(v : &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if v[i - 1] > v[i] {
            return false;
        }
    }
    true
}

fn partition(v: &mut [i32]) -> usize {
    let pivot = v.len() - 1;
    let mut k = 0;
    for i in 0 .. pivot {
        if v[i] <= v[pivot] {
            v.swap(i, k);
            k += 1;
        }
    }
    v.swap(k, pivot);
    k
}

fn quick_sort(v: &mut [i32]) {
    if v.len() <= 1 {
        return;
    }
    let i = partition(v);
    quick_sort(&mut v[0 .. i]);
    quick_sort(&mut v[i ..]);
}

fn binsearch(v: &[i32], target: i32) -> Result<usize,usize> {
    let mut lo = 0;
    let mut hi = v.len();
    while lo != hi {
        let mid = lo + (hi - lo) / 2;
        if v[mid] < target {
            lo = mid + 1;
        } else if v[mid] > target {
            hi = mid;
        } else {
            return Ok(mid);
        }
    }
    Err(0)
}

fn main() {

    let i = binsearch(&[0,1,2,3,4], 3).unwrap();
    println!("i == {}", i);
    assert!(i == 3);

    for _ in 0..100 {
        let mut v = random_vec();
        let target = v[0];
        quick_sort(&mut v);
        assert!(is_sorted(&v));
        let i = binsearch(&v, target).unwrap();
        assert!(v[i] == target);
    }
    println!("Passed");
}
