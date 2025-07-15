#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod capped_vec;

//pub struct capped_vec::CappedVec;

pub mod capped_map;

pub mod capped_set;

pub mod capped_deque;

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