extern crate rand;

use rand::Rng;
use std::mem;
// use std::cmp::Ordering;

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

// Doesn't work, as we end up creating two new mutable references
// to vector when we call this, and that violates the "only 1 mutable ref"
// rule....
//
// fn swap<'a>(x: &'a mut i32, y: &'a mut i32) {
//     let t = *x;
//     *x = *y;
//     *y = t;
// }

fn swap(v: &mut [i32], i: usize, k: usize) {
    let t = v[k];
    v[k] = v[i];
    v[i] = t;    
}

fn partition(v: &mut [i32]) -> usize {
    let pivot = v.len() - 1;
    let mut k = 0;
    for i in 0 .. pivot {
        if v[i] <= v[pivot] {
            swap(v, i, k);
            k += 1;
        }
    }
    swap(v, k, pivot);
    k
}

fn quick_sort(v: &mut [i32]) {
    if v.len() <= 1 {
        return;
    }
    // println!("quick_sort({:?})", v);
    let i = partition(v);
    // println!("partitioned i={} v={:?}", i, v);
    quick_sort(&mut v[0 .. i]);
    quick_sort(&mut v[i ..]);
}

fn binsearch(v: &[i32], target: i32) -> usize {
    let mut lo = 0;
    let mut hi = v.len();
    while lo != hi {
        let mid = lo + (hi - lo) / 2;
        if v[mid] < target {
            lo = mid + 1;
        } else if v[mid] > target {
            hi = mid;
        } else {
            return mid;
        }
    }
    0
}

fn binsearch_(v: &[i32], target: i32) -> usize {
    let mut v2 = v;
    while v2.len() > 0 {
        let mid = v2.len() / 2;
        if v2[mid] < target {
            v2 = v2[mid + 1 .. ];
        } else if v2[mid] > target {
            v2 = v2[0 .. mid];
        } else {
            return mid;
        }        
    }
    0
}

fn main() {
    for _ in 0..100 {
        let mut v = random_vec();
        let target = v[0];
        quick_sort(&mut v);
        assert!(is_sorted(&v));
        let i = binsearch_(&v, target);
        assert!(v[i] == target);
    }
    println!("Passed");
    let mut x = 4;
    let mut y = 6;
    mem::swap(&mut x, &mut y);
    println!("x={} y={}", x, y);
    assert!(x == 6);
    assert!(y == 4);

}
