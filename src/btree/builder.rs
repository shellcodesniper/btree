use std::marker::PhantomData;

use super::node::Node;

pub struct BTreeBuilder<K, V> where K: Ord, {
  pub max_key_size: usize,
  pub max_page_size: usize,
  pub page_header_size: usize,
  pub root_path: String,
  pub root_node: Option<Node<K, V>>,
  pub _markers: PhantomData<(K, V)>,
}

impl<K: Ord, V> Default for BTreeBuilder<K, V> {
  fn default() -> Self {
    Self {
      max_key_size: 255,
      max_page_size: 4096,
      page_header_size: 16,
      root_path: String::new(),
      _markers: PhantomData,
    }
  }
}

impl<K: Ord, V> BTreeBuilder<K, V> {
  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_string(),
      ..Default::default()
    }
  }

  pub fn set_max_key_size(&mut self, max_key_size: usize) -> &mut Self {
    self.max_key_size = max_key_size;
    self
  }

  pub fn set_max_page_size(&mut self, max_page_size: usize) -> &mut Self {
    self.max_page_size = max_page_size;
    self
  }

  pub fn set_page_header_size(&mut self, page_header_size: usize) -> &mut Self {
    self.page_header_size = page_header_size;
    self
  }

  pub fn build(&self) -> Node<K, V> {
    
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut builder = BTreeBuilder::<usize, usize>::new("test");
    builder.set_max_key_size(255);
    builder.set_max_page_size(4096);
    builder.set_page_header_size(16);
  }

  #[test]
  fn it_works2() {
    let builder = BTreeBuilder::<u64, String>::new("test").set_max_key_size(233);
  }

}
