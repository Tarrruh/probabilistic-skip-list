use std::io::{self, Read};
use std::{fmt};
use std::fmt::Formatter;
use std::ops::Neg;

pub trait KeyVal {
    type Key: Ord;
    type Value;
    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
}

impl<T: Ord> KeyVal for T {
    type Key = T;
    type Value = T;
    fn key(&self) -> &Self::Key {
        self
    }
    fn value(&self) -> &Self::Value {
        self
    }
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub(crate) enum Bound<T> {
    NegInf,
    Value(T),
    PosInf
}

impl<T> Bound<T> {
    pub fn value(&self) -> &T {
        match self {
            Bound::NegInf => panic!("Accessing negative sentinel"),
            Bound::Value(T) => T,
            Bound::PosInf => panic!("Accessing positive sentinel")
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyValuePair<K, V>(pub K, pub V);

impl<K: Ord, V> KeyVal for KeyValuePair<K, V> {
    type Key = K;
    type Value = V;
    fn key(&self) -> &Self::Key {
        &self.0
    }
    fn value(&self) -> &Self::Value {
        &self.1
    }
}
#[derive(Debug, Clone)]
pub struct SkipListNode<T: KeyVal + Clone> {
    data: Bound<T>,
    forwards: Vec<Box<Option<SkipListNode<T>>>>,
}

impl<T: KeyVal + Clone> SkipListNode<T> {
    pub fn new(data: T, level: usize) -> Self {
        SkipListNode {
            data: Bound::Value(data),
            forwards: vec![Box::new(None); level],
        }
    }

    pub fn get_data(&self) -> &T {
        self.data.value()
    }

    pub fn get_level_len(&self) -> usize {
        self.forwards.len()
    }

    pub fn get_mut_forwards(&mut self) -> &mut Vec<Box<Option<SkipListNode<T>>>> {
        &mut self.forwards
    }

}

pub struct ProbabilisticSkipList<T: KeyVal + Clone> {
    length: i32,
    head: Option<SkipListNode<T>>
}
pub fn main() {
    println!("{}", Bound::NegInf < Bound::Value(10))
}
