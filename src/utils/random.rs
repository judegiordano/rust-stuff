use rand::Rng;

pub fn random_number(lower: u32, upper: u32) -> u32 {
    // inclusive lower bound, exclusive upper bound
    // return rand::thread_rng().gen_range(1..101);
    return rand::thread_rng().gen_range(lower..=upper);
}
