//
// use csv::Reader;
//
// use crate::probabilistic_skip_list::*;
// fn main() {
//     let mut skip_list: ProbabilisticSkipList<KeyValuePair<String, i32>> = ProbabilisticSkipList::new(1f32/2f32);
//     skip_list.insert(KeyValuePair("apple".to_string(), 10));
//     skip_list.insert(KeyValuePair("ban".to_string(), 10));
//     skip_list.insert(KeyValuePair("banana".to_string(), 10));
//     println!("{:?}", skip_list);
//
//     println!("{:?}", skip_list.search("apple".to_string()));
//     println!("{:?}", skip_list.delete("apple".to_string()));
//     println!("{:?}", skip_list);
//     skip_list.insert(KeyValuePair("zaza".to_string(), 10));
//     println!("{:?}", skip_list);
//     println!("{:?}", skip_list.free_list);
// }
