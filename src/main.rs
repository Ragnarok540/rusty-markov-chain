extern crate rand;

use std::collections::HashMap;
use rand::Rng;

fn main() {

  let mut rng = rand::thread_rng();
  let string = "the theremin is theirs or is it not theirs";
  let mut m = 0;
  let mut n = 3;
  let mut map = HashMap::new();
    
  while n < string.len() {

    let vec = map.entry(&string[m..n]).or_insert(Vec::new());
    vec.push(&string[n..n + 1]);
    m += 1;
    n += 1;

  }

  //println!("{:?}", map);
  
  m = 0;
  n = 3;
  
  let current = &string[m..n];
  //let result = current;
  let posibilities = map.get(current).unwrap();
    
  println!("{:?}", posibilities[rng.gen_range(0, posibilities.len())]);
  println!("{:?}", posibilities[rng.gen_range(0, posibilities.len())]);
  println!("{:?}", posibilities[rng.gen_range(0, posibilities.len())]);
  println!("{:?}", posibilities[rng.gen_range(0, posibilities.len())]);

}
