use std::io::{self, Read};
use std::random;


pub trait KeyVal {
    type Key: Ord;
    fn key(&self) -> &Self::Key;
}

impl<T: Ord> KeyVal for T {
    type Key = T;
    fn key(&self) -> &Self::Key {
        self
    }
}

impl<K: Ord, V> KeyVal for (K, V) {
    type Key = K;
    fn key(&self) -> &Self::Key {
        &self.0
    }
}
pub struct SkipListNode<T: KeyVal> {

}

pub struct ProbabilisticSkipList<T> {
    length: i32,
    head: SkipListNode<T>
}
