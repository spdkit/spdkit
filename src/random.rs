// [[file:../spdkit.note::*imports][imports:1]]
use std::sync::Mutex;

use crate::common::*;
// imports:1 ends here

// [[file:../spdkit.note::6594d8a9][6594d8a9]]
pub use rand::prelude::*;

// To not cause trouble, make sure one RNG per thread
fn random_seed_u64() -> u64 {
    rand::random::<u64>()
}

fn get_rng_with_seed(seed: Option<u64>) -> StdRng {
    let seed = seed.unwrap_or_else(|| rand::random::<u64>());
    info!("Initialize rng with seed {}", seed);

    StdRng::seed_from_u64(seed)
}

lazy_static! {
    pub static ref RNG: Mutex<StdRng> = {
        let mut r = get_rng_with_seed(None);
        Mutex::new(r)
    };
}

#[macro_export]
macro_rules! get_rng {
    () => {
        RNG.lock().unwrap()
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_rng() {
        use crate::random::*;

        let mut rng = get_rng!();
        rng.gen::<i32>();
        assert_eq!(rng.gen_range(0..1), 0);
    }
}
// 6594d8a9 ends here
