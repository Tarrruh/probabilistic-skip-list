
mod probabilistic_skip_list;
mod test;
use std::time;
use std::time::{Duration, Instant};
use csv;
use serde::Deserialize;

//Index,Customer Id,First Name,Last Name,Company,City,Country,Phone 1,Phone 2,Email,Subscription Date,Website
#[derive(Debug, Deserialize, Clone)]
struct Customer {
    index: i32,
    customer_id: String,
    first_name: String,
    last_name: String,
    company: String,
    city: String,
    country:String,
    phone1: String,
    phone2:String,
    email: String,
    subscription_date: String,
    website: String,
}

use crate::probabilistic_skip_list::*;
fn main() {

    let curr = Instant::now();

    let mut skip_list: ProbabilisticSkipList<KeyValuePair<String, Customer>> = ProbabilisticSkipList::new(1f32/2f32);
    let csv_reader = csv::Reader::from_path("./src/customers-2mil.csv");
    if let Ok(mut records) = csv_reader {
        for result in records.deserialize() {
            let customer: Customer = result.expect("Csv read error");
            skip_list.insert(KeyValuePair(customer.customer_id.clone(), customer)).expect("PANICCCC!!!!!!!!!! get a better csv!!!!");
        }
    }
    println!("Time spent inserting - {:?}",curr.elapsed());


    let time_search = Instant::now();
    println!("{:#?}",skip_list.search("ecEe106aFaA0EAC".to_string()));
    println!("Time spent searching - {:?}",time_search.elapsed());

    println!("{:?}",skip_list.length());

    skip_list.delete("7ac9D288dBb129f".to_string());
    println!("{:#?}",skip_list.search("7ac9D288dBb129f".to_string()));
    
    // println!("{:?}",skip_list.get_nodes_list().get(0));

}
