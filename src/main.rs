use std::collections::HashMap;

fn main() {
    
    let string = "the theremin is theirs or is it not theirs";
    let mut n = 0;
    let mut m = 3;    
    let mut map = HashMap::new();
    
    while m <= string.len() {	  
	  let counter = map.entry(&string[n..m]).or_insert(0);
	  *counter += 1;
      n += 1;
      m += 1;
    }
    
    println!("{:?}", map);
    
}
