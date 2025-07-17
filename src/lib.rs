#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod capped_vec;

pub mod capped_map;

pub mod capped_set;

pub mod capped_deque;

pub mod capped_queue;

//Re-exports

pub use capped_deque::CappedDeque;

pub use capped_map::CappedMap;

pub use capped_queue::CappedQueue;

pub use capped_set::CappedSet;

pub use capped_vec::CappedVec;

/*
mod capped_vec;

pub use capped_vec::*;

#[cfg(test)]
mod capped_vec_tests;
*/

/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/