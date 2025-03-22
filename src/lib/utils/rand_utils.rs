use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn random_fill_by_range(buf: &mut [char]) {
    let mut rng = StdRng::from_os_rng();
    for i in buf {
        *i = rng.random::<char>();
    }
}
