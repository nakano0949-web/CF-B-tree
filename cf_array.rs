pub fn to_cf(mut m: u128, mut n: u128) -> Vec<u64> {
    let mut cf = Vec::new();
    cf.push((m / n) as u64);
    m %= n;

    while m != 0 {
        let q = n / m;
        let r = n % m;
        cf.push(q as u64);
        n = m;
        m = r;
    }
    cf
}

pub fn f64_to_cf(val: f64) -> Vec<u64> {
    let precision = 1_000_000_000_000u128;
    let m = (val * precision as f64).round() as u128;
    to_cf(m, precision)
}
