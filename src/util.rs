use rand::{rngs::StdRng, SeedableRng};

pub fn create_seeded_rng() -> StdRng {
    StdRng::seed_from_u64(123456789)
}
