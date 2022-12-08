use std::env;
use lazy_static::lazy_static;

mod types;
pub mod error;
pub mod btree;


lazy_static! {
  static ref MAX_KEY_SIZE: usize = env!("MAX_KEY_SIZE").parse().unwrap_or_else(|_| {
    panic!("MAX_KEY_SIZE must be a positive integer");
  });
  static ref MAX_PAGE_SIZE:  usize = env!("MAX_PAGE_SIZE").parse().unwrap_or_else(|_| {
    panic!("MAX_PAGE_SIZE must be a positive integer");
  });
  static ref PAGE_HEADER_SIZE: usize = env!("PAGE_HEADER_SIZE").parse().unwrap_or_else(|_| {
    panic!("PAGE_HEADER_SIZE must be a positive integer");
  });
}

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
