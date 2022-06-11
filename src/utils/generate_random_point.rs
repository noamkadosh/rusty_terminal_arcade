use rand::Rng;
use std::ops::Range;

pub fn generate_random_point(range: (usize, usize)) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    return (
        rng.gen_range::<usize, Range<usize>>(0..range.0),
        rng.gen_range::<usize, Range<usize>>(0..range.1),
    );
}
