extern crate time;

use std::collections::HashMap;
use std::cmp::max;
use time::PreciseTime;

fn main() {
    let m = 3;

    for n in [20, 25, 30].iter() {
        println!("n: {}", n);

        let s0 = PreciseTime::now();
        let res0 = ack_with_cache(m, *n);
        let e0 = PreciseTime::now();
        println!("ack_with_cache -> {} took {}", res0, s0.to(e0));

        let s1 = PreciseTime::now();
        let res1 = ack_with_struct(m, *n);
        let e1 = PreciseTime::now();
        println!("ack_with_struct -> {} took {}", res1, s1.to(e1));

        let s2 = PreciseTime::now();
        let res2 = ack_with_vector(m, *n);
        let e2 = PreciseTime::now();
        println!("ack_with_vector -> {} took {}", res2, s2.to(e2));

        let s3 = PreciseTime::now();
        let res3 = ack_with_array(m, *n);
        let e3 = PreciseTime::now();
        println!("ack_with_array -> {} took {}", res3, s3.to(e3));
    }
}

#[allow(dead_code)]
fn naive_ack(m: u64, n: u64) -> u64 {
    match (m, n) {
        (0, n) => n + 1,
        (m, 0) => naive_ack(m - 1, 1),
        (m, n) => naive_ack(m - 1, naive_ack(m, n - 1)),
    }
}

fn ack_with_cache(m: u64, n: u64) -> u64 {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    _ack_with_cache(&mut cache, m, n)
}

fn _ack_with_cache(cache: &mut HashMap<(u64, u64), u64>, m: u64, n: u64) -> u64 {
    match (m, n) {
        (0, n) => n + 1,
        (1, n) => n + 2,
        (m, 0) => _ack_with_cache(cache, m - 1, 1),
        (m, 1) => {
            let n = _ack_with_cache(cache, m - 1, 1);
            _ack_with_cache(cache, m - 1, n)
        }
        (m, n) => {
            if cache.contains_key(&(m, n)) {
                *cache.get(&(m, n)).unwrap()
            } else {
                let s = _ack_with_cache(cache, m, n - 2);
                let t = _ack_with_cache(cache, m, n - 1);

                let res = (s..(t + 1)).fold(0, |acc, x| _ack_comparator(cache, m, acc, x));
                cache.insert((m, n), res);
                res
            }
        }
    }
}

fn _ack_comparator(cache: &mut HashMap<(u64, u64), u64>, m: u64, acc: u64, x: u64) -> u64 {
    let c = _ack_with_cache(cache, m - 1, x);
    max(acc, c)
}

#[derive(Clone)]
struct Record {
    result: u64,
    previous_result: u64,
    cache: Option<Box<Record>>,
}

impl Record {
    fn new(result: u64, previous_result: u64, cache: Option<Box<Record>>) -> Record {
        Record { result, previous_result, cache }
    }
}

fn ack_with_struct(m: u64, n: u64) -> u64 {
    let mut cache = Record::new(1, 0, None);
    for m_builder in 0..m {
        cache = _ack_with_struct(m_builder, 1, cache)
    }
    for n_builder in 1..(n + 1) {
        cache = _ack_with_struct(m, n_builder, cache)
    }
    cache.result
}

fn _ack_with_struct(m: u64, n: u64, current_cache: Record) -> Record {
    if m == 0 {
        Record::new(n + 1, 0, None)
    } else if m == 1 {
        Record::new(n + 2, 0, None)
    } else if n == 0 {
        let mut new_cache = Record::new(1, 0, None);
        for m_builder in 0..m {
            new_cache = _ack_with_struct(m_builder, 1, new_cache);
        }
        new_cache
    } else if n == 1 {
        let cache_result = current_cache.result;
        let mut new_cache = current_cache;
        for n_builder in 2..(cache_result + 1) {
            new_cache = _ack_with_struct(m - 1, n_builder, new_cache);
        }
        Record::new(new_cache.result, cache_result, Some(Box::new(new_cache)))
    } else {
        let cache_result = current_cache.result;
        let mut new_cache = *current_cache.cache.unwrap();
        for n_builder in (current_cache.previous_result + 1)..(current_cache.result + 1) {
            new_cache = _ack_with_struct(m - 1, n_builder, new_cache);
        }
        Record::new(new_cache.result, cache_result, Some(Box::new(new_cache)))
    }
}

fn ack_with_vector(m: u64, n: u64) -> u64 {
    let mut v: Vec<u64> = Vec::with_capacity(6);
    v.push(1);
    for k in 0..m {
        v = _ack_with_vector(k, 1, v)
    }
    for j in 1..(n + 1) {
        v = _ack_with_vector(m, j, v)
    }
    v[0]
}

fn _ack_with_vector(m: u64, n: u64, r_use: Vec<u64>) -> Vec<u64> {
    if m == 0 {
        vec![n + 1]
    } else if m == 1 {
        vec![n + 2]
    } else if n == 0 {
        (0..m).fold(vec![1], |v, k| _ack_with_vector(k, 1, v))
    } else if n == 1 {
        let v11 = r_use[0];
        let mut v2 = r_use.clone();
        for k in 2..(v11 + 1) {
            v2 = _ack_with_vector(m - 1, k, v2);
        }
        let v3 = v2[0];
        v2.insert(0, v3);
        v2.insert(1, v11);
        v2
    } else {
        let v11 = r_use[0];
        let mut v2 = r_use[2..].to_owned();
        for k in (r_use[1] + 1)..(v11 + 1) {
            v2 = _ack_with_vector(m - 1, k, v2);
        }
        let v3 = v2[0];
        v2.insert(0, v3);
        v2.insert(1, v11);
        v2
    }
}

fn ack_with_array(m: u64, n: u64) -> u64 {
    let mut cache: [u64; 6] = [0; 6];
    cache[0] = 1;
    for m_builder in 0..m {
        cache = _ack_with_array(m_builder, 1, cache)
    }
    for n_builder in 1..(n + 1) {
        cache = _ack_with_array(m, n_builder, cache)
    }
    cache[0]
}

fn _ack_with_array(m: u64, n: u64, mut cache: [u64; 6]) -> [u64; 6] {
    if m == 0 {
        [n + 1, 0, 0, 0, 0, 0]
    } else if m == 1 {
        [n + 2, 0, 0, 0, 0, 0]
    } else if n == 0 {
        let mut new_cache = [1, 0, 0, 0, 0, 0];
        for m_builder in 0..m {
            new_cache = _ack_with_array(m_builder, 1, new_cache);
        }
        new_cache
    } else if n == 1 {
        let previous_result = cache[0];
        for n_builder in 2..(previous_result + 1) {
            cache = _ack_with_array(m - 1, n_builder, cache);
        }
        [cache[0], previous_result, cache[0], cache[1], cache[2], cache[3]]
    } else {
        let previous_result = cache[0];
        let mut new_cache = [cache[2], cache[3], cache[4], cache[5], 0, 0];
        for n_builder in (cache[1] + 1)..(previous_result + 1) {
            new_cache = _ack_with_array(m - 1, n_builder, new_cache);
        }
        [new_cache[0], previous_result, new_cache[0], new_cache[1], new_cache[2], new_cache[3]]
    }
}
