
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



    //10, 20, 50, 100, 200
    // 5 csv files
    // min 10 rows
    // min 3 cols
    // read from csv

    let mut skip_list: ProbabilisticSkipList<KeyValuePair<String, i32>> = ProbabilisticSkipList::new(1f32/2f32);
    skip_list.insert(KeyValuePair("apple".to_string(), 10));
    skip_list.insert(KeyValuePair("ban".to_string(), 10));
    skip_list.insert(KeyValuePair("banana".to_string(), 10));
    println!("{:?}", skip_list);

    println!("{:?}", skip_list.search("apple".to_string()));
    println!("{:?}", skip_list.delete("apple".to_string()));
    println!("{:?}", skip_list);
    skip_list.insert(KeyValuePair("zaza".to_string(), 10));
    println!("{:?}", skip_list);
    println!("{:?}", skip_list.free_list);
}
