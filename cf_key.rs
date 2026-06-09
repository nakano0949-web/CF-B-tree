#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct CFKey(pub Vec<u8>);

pub fn encode_cf_binary_be(cf: &[u64]) -> CFKey {
    let mut out = Vec::with_capacity(cf.len() * 8);
    const MAX: u64 = std::u64::MAX;

    for (i, &x) in cf.iter().enumerate() {
        let v = if i % 2 == 1 { MAX - x } else { x };
        out.extend_from_slice(&v.to_be_bytes());
    }
    CFKey(out)
}
