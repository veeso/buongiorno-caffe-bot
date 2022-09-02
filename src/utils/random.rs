//! # Random
//!
//! Random utils

use rand::Rng;

/// Choose a random element from `choices`
pub fn choice<T>(choices: &[T]) -> &T {
    let mut rng = rand::thread_rng();
    &choices[rng.gen_range(0..choices.len())]
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_get_a_random_choice() {
        let choices = vec!["a", "b", "c"];
        assert!(choices.contains(choice(&choices)));
    }
}
