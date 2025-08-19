use std::io::{self, Read};
use std::{fmt};
use std::cmp::*;
use std::fmt::{Error, Formatter};
use std::ops::Neg;
use std::rc::Rc;
use rand::{random, Rng};

const MAX_LEVEL: i32 = 20; //maybe change it to 16 idk compare later
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

impl<T: KeyVal> Bound<T> {
    pub fn value(&self) -> &T {
        match self {
            Bound::NegInf => panic!("Accessing negative sentinel"),
            Bound::Value(t) => t,
            Bound::PosInf => panic!("Accessing positive sentinel"),
            Bound::Null => panic!("Accessing null value")
        }
    }

    pub fn cmp_key(&self, key: &T::Key) ->Ordering
    where
        T::Key: Ord,
    {
        match self {
            Bound::NegInf => Ordering::Less,
            Bound::PosInf => Ordering::Greater,
            Bound::Value(t) => t.key().cmp(key),
            Bound::Null => panic!("Comparing with null bound"),
        }
    }

    pub fn eq_key(&self, key: &T::Key) -> bool
    where
        T::Key: PartialEq,
    {
        match self {
            Bound::Value(t) => *t.key() == *key,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyValuePair<K, V>(pub K, pub V);

impl<K: Ord + PartialOrd, V> KeyVal for KeyValuePair<K, V> {
    type Key = K;
    type Value = V;
    fn key(&self) -> &Self::Key {
        &self.0
    }
    fn value(&self) -> &Self::Value {
        &self.1
    }
}

impl<K: PartialOrd, V: PartialOrd> PartialOrd for KeyValuePair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for KeyValuePair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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
            forwards: vec![None; level + 1usize],
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
            forwards: vec![None; level + 1usize],
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
    pub free_list: Vec<NodeID>,
    nodes: Vec<SkipListNode<T>>,
    promotion_chance: f32,
}

impl<T: KeyVal + Clone + PartialOrd> ProbabilisticSkipList<T> {

    pub fn new(promotion_chance: f32) -> Self {
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
            promotion_chance,
        }
    }

    pub fn insert(&mut self, data: T) -> Result<NodeID, &'static str>
    where
        T::Key: Ord + PartialEq + Clone,
    {
        if self.search(data.key().clone()).is_some() {
            return Err("Duplicate key insertion");
        }

        let index = self.allocate_index();
        let max_level = self.get_max_level();
        let node = SkipListNode::new(data.clone(), max_level);
        self.nodes[index] = node;

        let mut updates = vec![0; max_level + 1];
        let mut curr_level = MAX_LEVEL - 1;
        let mut curr_node = self.head;
        while curr_level >= 0 {
            if let Some(node_index) = self.nodes[curr_node].forwards[curr_level as usize] {
                let node_bound = &self.nodes[node_index].data;

                if node_bound.cmp_key(data.key()) > Ordering::Equal {
                    if updates.len() > curr_level as usize {
                        updates[curr_level as usize] = curr_node;
                    }
                    curr_level -= 1;
                } else {
                    curr_node = node_index;
                }
            } else {
                updates[curr_level as usize] = curr_node;
                curr_level -= 1;
            }
        }

        for lvl in 0..=max_level {
            let temp = self.nodes[updates[lvl]].forwards[lvl];
            self.nodes[index].forwards[lvl] = temp;
            self.nodes[updates[lvl]].forwards[lvl] = Some(index);
        }

        self.length += 1;
        Ok(index)
    }


    pub fn search(&self, key: T::Key) -> Option<T>
    where
        T::Key: Ord + PartialEq,
    {
        let mut curr_level = MAX_LEVEL - 1;
        let mut curr_node = self.head;
        while curr_level >= 0 {
            if let Some(node_index) = self.nodes[curr_node].forwards[curr_level as usize] {
                let node_bound = &self.nodes[node_index].data;

                match node_bound.cmp_key(&key) {
                    Ordering::Greater => {
                        curr_level -= 1;
                    }
                    Ordering::Equal => {
                        return Some(node_bound.value().clone());
                    }
                    Ordering::Less => {
                        curr_node = node_index;
                    }
                }
            } else {
                curr_level -= 1;
            }
        }
        None
    }


    pub fn delete(&mut self, data: T::Key) -> Option<T>
    where
        T::Key: Ord + PartialEq,
    {
        
        let mut curr_node = self.head;
        let mut curr_level = MAX_LEVEL - 1;
        let mut found = -1i32;
        
        while curr_level >= 0 {
            if let Some(node_index) = self.nodes[curr_node].forwards[curr_level as usize] {
                let node_bound = &self.nodes[node_index].data;
                match node_bound.cmp_key(&data.key()) { 
                    Ordering::Less => {
                        curr_node = node_index
                    }
                    Ordering::Greater => {
                        curr_level -= 1;
                    }
                    Ordering::Equal => {
                        found = node_index as i32;
                        self.nodes[curr_node].forwards[curr_level as usize] = self.nodes[node_index].forwards[curr_level as usize].clone();
                    }
                }
            }
        }
        if found >= 0 {
            self.free_list.push(found as NodeID);
            self.length -= 1;
            return Some(self.nodes[found as usize].data.value().clone()) //return clone bc data might change
        }
        None
    }
    

    pub fn length(&self) -> usize {
        self.length
    }

    fn allocate_index(&mut self) -> NodeID {
        if let Some(id) = self.free_list.pop() {
            id
        }
        else {
            let id = self.nodes.len();
            self.nodes.push(SkipListNode::default());
            id
        }
    }

    fn get_max_level(&self) -> usize {
        let mut level = 0;
        let mut rng = rand::rng();
        while level < MAX_LEVEL as usize && rng.random::<f32>() < self.promotion_chance {
            level += 1;
        }
        level
    }

}
