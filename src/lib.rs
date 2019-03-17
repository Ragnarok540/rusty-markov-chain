extern crate rand;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Markov {
    pub text: String,
    pub size: usize,
    pub map: HashMap<String, Vec<String>>,
}

impl Markov {

    pub fn new() -> Markov {
        Markov {
            text: String::new(),
            size: 0,
            map: HashMap::new(),
        }
    }

    pub fn chain(&mut self, text: String, size: usize) {
        self.text = text;
        self.size = size;
        let mut i = 0;
        let mut j = self.size;

        while j < self.text.len() {
            let vec = self.map.entry(self.text[i..j].to_string()).or_insert(Vec::new());
            vec.push(self.text[j..j + 1].to_string());
            i += 1;
            j += 1;
        }
    }
    
    pub fn generate(&mut self, start: String, times: usize) {
        let mut i = 0;
        let mut rng = rand::thread_rng();
        let mut result = String::new(); 
        let mut current = start;
        
        result.push_str(&current);
        
        while i < times {
            let pos = self.map.get(&current);
            
            let next = match pos {
                Some(x) => &x[rng.gen_range(0, x.len())],
                None    => "",
            };
            
            result.push_str(&next);
            
            let len = result.len();
            
            current = result[len - self.size..len].to_string();
            i += 1;
        }
        
        println!("{:?}", result);
    }

}
