use std::env;
use lazy_static::lazy_static;

mod types;
pub mod error;
pub mod btree;

pub const BTREE_CAPACITY: usize = 4;
pub const MAX_DATA_PAGE_SIZE: usize = 4096;
pub const PAGE_HEADER_SIZE: usize = 4;

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}


/*
* @license MIT
* @author 2022 kuuwange
*/
