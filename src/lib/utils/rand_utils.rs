use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn random_fill_by_range(buf: &mut Vec<char>) {
    let mut rng = StdRng::from_os_rng();
    for i in buf {
        *i = rng.random::<char>();
    }
}

pub fn random_string_of_len(len: usize) -> String {
  let mut buf: Vec<char> = Vec::with_capacity(len);
  random_fill_by_range(&mut buf);
  buf.into_iter().collect()
}
