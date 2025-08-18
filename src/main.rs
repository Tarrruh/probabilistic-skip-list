
mod probabilistic_skip_list;

use crate::probabilistic_skip_list::*;
fn main() {
    // println!("{}", Bound::Value(5) < Bound::Value(10));
    // println!("{:?}", SkipListNode::new(5,3));
    // let a = KeyValuePair("tome string idk", KeyValuePair("String", 1));
    // let b = KeyValuePair("some string idk", KeyValuePair("String", 1));
    // println!("{}", a.key() < b.key());
    // println!("{:?}", SkipListNode::new(a, 5))
    // 
    
    let mut skip_list: ProbabilisticSkipList<i32> = ProbabilisticSkipList::new(1f32/2f32);
    println!("{:?}", skip_list);
    skip_list.insert(10);
    skip_list.insert(5);
    skip_list.insert(7);
    println!("{:?}", skip_list);
}
