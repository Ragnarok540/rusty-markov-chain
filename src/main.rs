//extern crate rand;

//use std::collections::HashMap;
//use rand::Rng;

extern crate rusty_markov_chain;
//use rusty_markov_chain::Possibilities;
use rusty_markov_chain::Markov;

fn main() {

let mut markov = Markov::new();

markov.chain("the theremin is theirs or is it not theirs".to_string(), 3);


println!("{:?}", markov.map.keys());

/*
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

  let mut result; 
  let mut current = &string[m..n];
  let mut resultado;
  

  result = String::from(current);
  
  
  while m < 10 {
	let possibilities = map.get(current).unwrap();
	let next = possibilities[rng.gen_range(0, possibilities.len())];
	
	
	
	result.push_str(&next);
	let len = result.len();
    println!("len: {}", len);
    
      resultado = &result.clone();
      current = &resultado.get(len - 3..len).unwrap();
    
	m += 1;  
	
	println!("{:?}", &result);
  }
  
  

  
  
    */
}
