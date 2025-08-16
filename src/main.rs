mod probablistic_skip_list;

fn main() {
    println!("{}", crate::probablistic_skip_list::Bound::NegInf < crate::probablistic_skip_list::Bound::Value(10));
}
