pub enum CFInput {
    Fraction(u128, u128),
    Float(f64),
    Key(CFKey),
}

pub fn to_cfkey(input: CFInput) -> CFKey {
    match input {
        CFInput::Fraction(m, n) => encode_cf_binary_be(&to_cf(m, n)),
        CFInput::Float(f)       => encode_cf_binary_be(&f64_to_cf(f)),
        CFInput::Key(k)         => k,
    }
}

pub struct CFIndex {
    map: BTreeMap<CFKey, (String, f64)>,
}

impl CFIndex {
    pub fn new() -> Self {
        Self { map: BTreeMap::new() }
    }

    pub fn insert_fraction(&mut self, m: u128, n: u128) {
        let key = encode_cf_binary_be(&to_cf(m, n));
        let val = m as f64 / n as f64;
        self.map.insert(key, (format!("{}/{}", m, n), val));
    }

    pub fn insert_float(&mut self, x: f64) {
        let key = encode_cf_binary_be(&f64_to_cf(x));
        self.map.insert(key, (format!("f64({})", x), x));
    }

    pub fn lower_bound(&self, x: CFInput)
        -> Option<(&CFKey, &(String, f64))>
    {
        let k = to_cfkey(x);
        self.map.range((Included(k), Unbounded)).next()
    }

    pub fn upper_bound(&self, x: CFInput)
        -> Option<(&CFKey, &(String, f64))>
    {
        let k = to_cfkey(x);
        self.map.range((Excluded(k), Unbounded)).next()
    }

    pub fn range_query<'a>(
        &'a self,
        lower: CFInput,
        upper: CFInput,
    ) -> impl Iterator<Item = (&'a CFKey, &'a (String, f64))> {
        let l = to_cfkey(lower);
        let u = to_cfkey(upper);
        self.map.range((Excluded(l), Excluded(u)))
    }
}
