use rand::Rng;

pub fn random_number() -> u32 {
    // inclusive lower bound, exclusive upper bound
    // return rand::thread_rng().gen_range(1..=100);
    return rand::thread_rng().gen_range(1..101);
}
