extern crate time;

use time::PreciseTime;

fn main() {
    let m = 3;

    for n in 1..20 {
        println!("n: {}", n);
        // let s1 = PreciseTime::now();
        // let res1 = ack(m, n);
        // let e1 = PreciseTime::now();
        // println!("ack -> {} took {}", res1, s1.to(e1));

        let s2 = PreciseTime::now();
        let res2 = a_opt(m, n);
        let e2 = PreciseTime::now();
        println!("opt -> {} took {}", res2, s2.to(e2));
    }
}

#[derive(Clone)]
struct Record {
    f: u64,
    s: u64,
    t: Option<Box<Record>>,
}

impl Record {
    fn new(f: u64, s: u64, t: Option<Box<Record>>) -> Record {
        Record { f: f, s: s, t: t }
    }
}

fn a_opt(i: u64, n: u64) -> u64 {
    let mut v = Record::new(1, 0, None);
    for k in 1..(i + 1) {
        v = a_use_p(k - 1, 1, v)
    }
    for j in 1..(n + 1) {
        v = a_use_p(i, j, v)
    }
    v.f
}

fn a_use_p(i: u64, n: u64, r_use: Record) -> Record {
    if i == 0 {
        Record::new(n + 1, 0, None)
    } else if n == 0 {
        let mut v = Record::new(1, 0, None);
        for k in 1..(i + 1) {
            v = a_use_p(k - 1, 1, v);
        }
        v
    } else if n - 1 == 0 {
        let v11 = r_use.f;
        let mut v2 = r_use.clone();
        for k in 2..(r_use.f + 1) {
            v2 = a_use_p(i - 1, k, v2);
        }
        Record::new(v2.f, v11, Some(Box::new(v2)))
    } else {
        let v11 = r_use.f;
        let mut v2 = *r_use.t.unwrap();
        for k in (r_use.s + 1)..(r_use.f + 1) {
            v2 = a_use_p(i - 1, k, v2);
        }
        Record::new(v2.f, v11, Some(Box::new(v2)))
    }
}
