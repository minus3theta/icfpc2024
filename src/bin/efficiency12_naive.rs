fn v5(v4: i64, v6: i64, v7: i64) -> i64 {
    if v6 == v4 {
        v7
    } else {
        let x = v3(v6);
        let second = if x > v6 - 1 && v4 % v6 == 0 {
            (v7 / x) * (x - 1)
        } else {
            v7
        };

        v5(v4, v6 + 1, second)
    }
}

fn v3(v4: i64) -> i64 {
    v4.min(1 + if v4 > 2 { v5(v4, 2, v4) } else { v4 })
}

fn main() {
    println!("{}", v3(1234567));
}
