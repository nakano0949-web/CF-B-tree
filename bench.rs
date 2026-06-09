fn bench_cf_btree(idx: &mut CFIndex) {
    use std::time::Instant;

    let n = 20_000;

    let t0 = Instant::now();
    for i in 1..=n {
        idx.insert_fraction(i * 1234567, i * 9876543 + 1);
    }
    println!("insert: {:?}", t0.elapsed());

    let t0 = Instant::now();
    for i in 1..=n {
        let _ = idx.lower_bound(CFInput::Fraction(i * 1234567, i * 9876543 + 1));
    }
    println!("lower_bound: {:?}", t0.elapsed());

    let t0 = Instant::now();
    for i in 1..=n {
        let _ = idx.range_query(
            CFInput::Fraction(i * 1234567, i * 9876543 + 1),
            CFInput::Fraction((i+5) * 1234567, (i+5) * 9876543 + 1),
        ).count();
    }
    println!("range_query: {:?}", t0.elapsed());
}
