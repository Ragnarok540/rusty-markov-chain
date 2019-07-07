extern crate rusty_markov_chain;

use rusty_markov_chain::Markov;

fn main() {
    let mut markov = Markov::new();
    //let string = String::from("the theremin is theirs or is it not theirs");
    //let string1 = String::from("to be or not to be that is the question");
    
    //markov.chain(string1, 3);
    
    //let string2 = String::from("below the thunders of the upper deep far far beneath in the abysmal sea his ancient dreamless uninvaded sleep the kraken sleepeth faintest sunlights flee about his shadowy sides above him swell huge sponges of millennial growth and height and far away into the sickly light from many a wondrous grot and secret cell unnumbered and enormous polypi winnow with giant arms the slumbering green there hath he lain for ages and will lie battening upon huge sea worms in his sleep until the latter fire shall heat the deep then once by man and angels to be seen in roaring he shall rise and on the surface die");
    
    //markov.chain(string2, 3);
    
    //println!("{:?}", markov);
    
    markov.chain_file(String::from("./file.txt"), 3);
    
    markov.generate(String::from("MIT"), 500);
}
