use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

fn divisors(x: i64) -> BTreeSet<i64> {
    let mut ret = BTreeSet::new();
    for i in 2.. {
        if i * i > x {
            break;
        }
        if x % i == 0 {
            ret.insert(i);
            ret.insert(x / i);
        }
    }
    ret
}

fn v5(v4: i64) -> i64 {
    let mut v7 = v4;
    for v6 in divisors(v4) {
        let x = v3(v6);
        if x > v6 - 1 {
            v7 = (v7 / x) * (x - 1);
        }
    }
    v7
}

thread_local! {
    static V3_CACHE: RefCell<HashMap<i64, i64>> = RefCell::new(HashMap::new());
}

fn v3(v4: i64) -> i64 {
    match V3_CACHE.with_borrow(|cache| cache.get(&v4).cloned()) {
        Some(ret) => ret,
        None => {
            let ret = v4.min(1 + if v4 > 2 { v5(v4) } else { v4 });
            V3_CACHE.with_borrow_mut(|cache| cache.insert(v4, ret));
            ret
        }
    }
}

fn main() {
    println!("{}", v3(1234567));
}
