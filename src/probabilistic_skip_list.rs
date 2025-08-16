use std::io::{self, Read};
use std::{fmt};
use std::fmt::Formatter;
use std::ops::Neg;
use std::rc::Rc;

const MAX_LEVEL: i32 = 20;
type NodeID = usize;
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
    PosInf,
    Null
}

impl<T> Bound<T> {
    pub fn value(&self) -> &T {
        match self {
            Bound::NegInf => panic!("Accessing negative sentinel"),
            Bound::Value(t) => t,
            Bound::PosInf => panic!("Accessing positive sentinel"),
            Bound::Null => panic!("Accessing null value")
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
    forwards: Vec<Option<NodeID>>,
}

impl<T: KeyVal + Clone> SkipListNode<T> {
    pub fn new(data: T, level: usize) -> Self {
        SkipListNode {
            data: Bound::Value(data),
            forwards: vec![None; level],
        }
    }

    pub fn default() -> Self {
        SkipListNode {
            data: Bound::Null,
            forwards: vec![None; 0],
        }
    }

    pub fn new_sentinel(bound: Bound<T>, level: usize) -> Self {
        SkipListNode {
            data: bound,
            forwards: vec![None; level],
        }
    }

    pub fn get_data(&self) -> &T {
        self.data.value()
    }

    pub fn get_level_len(&self) -> usize {
        self.forwards.len()
    }

    pub fn get_mut_forwards(&mut self) -> &mut Vec<Option<NodeID>>  {
        &mut self.forwards
    }

}

#[derive(Debug)]
pub struct ProbabilisticSkipList<T: KeyVal + Clone> {
    length: usize,
    head: NodeID,
    free_list: Vec<NodeID>,
    nodes: Vec<SkipListNode<T>>,
}

impl<T: KeyVal + Clone> ProbabilisticSkipList<T> {

    pub fn new() -> Self {
        let mut head = SkipListNode::new_sentinel(Bound::<T>::NegInf, MAX_LEVEL as usize);
        let tail = SkipListNode::new_sentinel(Bound::<T>::PosInf, MAX_LEVEL as usize);
        for i in 0..head.forwards.len() {
            head.forwards[i] = Some(1 as NodeID);
        }
        ProbabilisticSkipList{
            length: 0,
            head: 0,
            free_list: vec![],
            nodes: vec![head, tail],
        }
    }

}
