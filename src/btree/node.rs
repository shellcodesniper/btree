use std::{
  boxed::Box,
  marker::{ PhantomData },
  mem::MaybeUninit,
  ptr::{ NonNull, self },
};

/**
* InternalNode들은 메모리에 로딩되며, 필요시에 읽어오는 방식으로 생각함.
* NOTE : 접근순서 : NodeRef -> InternalNode -> LeafNode
*/

use crate::BTREE_CAPACITY;

/// 노드와, 그 이하 자식들을 관리하는 포인터구조체라는느낌?
pub struct InternalNode<K, V> {
  /// 노드
  data: LeafNode<K, V>,

  /// 자식 노드들의 포인터
  edges: [MaybeUninit<BoxedNode<K, V>>; BTREE_CAPACITY + 1],
}

impl<K, V> InternalNode<K, V> {
  unsafe fn new(key: K, value: V) -> Box<Self> {
    unsafe {
      let mut edges = MaybeUninit::uninit();
      let edges = edges.assume_init();
      Box::new(Self {
        data: LeafNode::new(key, value),
        edges,
      })
    }
  }

}


/// 노드 구조체
pub struct LeafNode<K, V> {
  parent: Option<NonNull<InternalNode<K, V>>>,
  parent_idx: MaybeUninit<u64>,

  len: u16,

  keys: [MaybeUninit<K>; BTREE_CAPACITY],
  vals: [MaybeUninit<V>; BTREE_CAPACITY],
}

impl<K, V> LeafNode<K, V> {
    /// Initializes a new `LeafNode` in-place.
  unsafe fn init(this: *mut Self) {
    // As a general policy, we leave fields uninitialized if they can be, as this should
    // be both slightly faster and easier to track in Valgrind.
    unsafe {
      // parent_idx, keys, and vals are all MaybeUninit
      ptr::addr_of_mut!((*this).parent).write(None);
      ptr::addr_of_mut!((*this).len).write(0);
    }
  }

    /// Creates a new boxed `LeafNode`.
  fn new() -> Box<Self, _> {
    unsafe {
      let leaf = Box::new();
      LeafNode::init(leaf.as_mut_ptr());
      leaf.assume_init()
    }
  }
}















pub struct NodeRef<BorrowType, K, V, Type> {
  height: usize,
  node: NonNull<LeafNode<K, V>>,
  _marker: PhantomData<(BorrowType, Type)>,
}











pub type Root<K, V> = NodeRef<marker::Owned, K, V, marker::LeafOrInternal>;

impl<'a, K: 'a, V: 'a, Type> Copy for NodeRef<marker::Immut<'a>, K, V, Type> {}
impl<'a, K: 'a, V: 'a, Type> Clone for NodeRef<marker::Immut<'a>, K, V, Type> {
    fn clone(&self) -> Self {
        *self
    }
}





unsafe impl<BorrowType, K: Sync, V: Sync, Type> Sync for NodeRef<BorrowType, K, V, Type> {}

unsafe impl<K: Sync, V: Sync, Type> Send for NodeRef<marker::Immut<'_>, K, V, Type> {}
unsafe impl<K: Send, V: Send, Type> Send for NodeRef<marker::Mut<'_>, K, V, Type> {}
unsafe impl<K: Send, V: Send, Type> Send for NodeRef<marker::ValMut<'_>, K, V, Type> {}
unsafe impl<K: Send, V: Send, Type> Send for NodeRef<marker::Owned, K, V, Type> {}
unsafe impl<K: Send, V: Send, Type> Send for NodeRef<marker::Dying, K, V, Type> {}










type BoxedNode<K, V> = Box<InternalNode<K, V>>;










pub enum ForceResult<Leaf, Internal> {
    Leaf(Leaf),
    Internal(Internal),
}






pub mod marker {
    use core::marker::PhantomData;

    pub enum Leaf {}
    pub enum Internal {}
    pub enum LeafOrInternal {}

    pub enum Owned {}
    pub enum Dying {}
    pub struct Immut<'a>(PhantomData<&'a ()>);
    pub struct Mut<'a>(PhantomData<&'a mut ()>);
    pub struct ValMut<'a>(PhantomData<&'a mut ()>);

    pub trait BorrowType {
        // If node references of this borrow type allow traversing to other
        // nodes in the tree, this constant can be evaluated. Thus reading it
        // serves as a compile-time assertion.
        const TRAVERSAL_PERMIT: () = ();
    }
    impl BorrowType for Owned {
        // Reject evaluation, because traversal isn't needed. Instead traversal
        // happens using the result of `borrow_mut`.
        // By disabling traversal, and only creating new references to roots,
        // we know that every reference of the `Owned` type is to a root node.
        const TRAVERSAL_PERMIT: () = panic!();
    }
    impl BorrowType for Dying {}
    impl<'a> BorrowType for Immut<'a> {}
    impl<'a> BorrowType for Mut<'a> {}
    impl<'a> BorrowType for ValMut<'a> {}

    pub enum KV {}
    pub enum Edge {}
}
