use std::collections::HashMap;

pub struct Possibilities {
    pub vec: Vec<String>,	
}

impl Possibilities {

    pub fn new() -> Possibilities {
        Possibilities {
           vec: Vec::new(),
        }
    }

}

pub struct Markov {
	pub text: String,
    pub map: HashMap<String, Possibilities>,
}

impl Markov {

    pub fn new() -> Markov {
        Markov {
		    text: String::new(),
            map: HashMap::new(),
        }
    }

    pub fn chain(&mut self, text: String, n: usize) {
		self.text = text;
		
    	let mut i = 0;
    	let mut j = n;
  
    	while j < self.text.len() {
			
			let pos = self.map.entry(self.text[i..j].to_string()).or_insert(Possibilities::new());
			pos.vec.push(self.text[j..j + 1].to_string());
			
			i += 1;
			j += 1;
		}
	
    }

}
