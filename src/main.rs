mod probabilistic_skip_list_complicated_but_ill_finish_it_sometime;
mod probabilistic_skip_list;

use crate::probabilistic_skip_list_complicated_but_ill_finish_it_sometime::*;
fn main() {
    println!("{}", Bound::Value(5) < Bound::Value(10));
    println!("{:?}", SkipListNode::new(5,3));
    let a = KeyValuePair("tome string idk", KeyValuePair("String", 1));
    let b = KeyValuePair("some string idk", KeyValuePair("String", 1));
    println!("{}", a.key() < b.key());
    println!("{:?}", SkipListNode::new(a, 5))
}
