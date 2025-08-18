use std::io::{self, Read};
use std::{fmt};
use std::cmp::max;
use std::fmt::Formatter;
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
    free_list: Vec<NodeID>,
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

    pub fn insert(&mut self, data: T) -> Result<NodeID, &'static str> {
        let index = self.allocate_index();
        let max_level = self.get_max_level();
        let mut node = SkipListNode::new(data.clone(), max_level as usize);
        self.nodes[index] = node;
        //store index in vec of size max_level
        //then store index of that node in index in list
        //update 4 level to new node so store it in in index 3.
        // ok take level from 0 instead to save headachee
        // do it before you forget
        //size should be level + 1
        //add equal check
        //youll have to update same amount of pointers as the max level + 1
        let mut updates = vec![0; max_level as usize + 1usize];
        let mut curr_level = MAX_LEVEL - 1;
        let mut curr_node = 0;
        while curr_level >= 0 {

            if let Some(node_index) = self.nodes.get(curr_node).expect("We shouldn't be here :( fix later").forwards[curr_level as usize] {
                if self.nodes.get(node_index).expect("Invalid index given here").data > Bound::Value(data.clone()) {
                    if updates.len() > curr_level as usize {
                        updates[curr_level as usize] = curr_node as i32;
                    }
                    curr_level -= 1;

                }
                else {
                    curr_node = self.nodes[curr_node].forwards[curr_level as usize].expect("Invalid id given")
                }
            }
            else {
                panic!("Shouldn't reach here");
            }
        }
        for idx in 0..updates.len() {
            let temp = self.nodes[updates[idx] as usize].forwards[idx];
            self.nodes[index].forwards[idx] = temp;
            self.nodes[updates[idx] as usize].forwards[idx] = Some(index);
        }
        println!("{:?}", updates);
        Ok(index)
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
    
    fn get_max_level(&self) -> i32 {
        let mut level = 0;
        let mut rng = rand::rng();
        while rng.random() {
            level += 1;
        }
        level
    }
 }
