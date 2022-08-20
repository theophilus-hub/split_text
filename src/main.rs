mod my_text_lib;
use my_text_lib::{wordify, breakify};


fn main() {
    let text = String::from("where can i see my broken lines in this sentence");
    let words: Vec<String> = wordify(text);
    let vec = vec![4,2];

    println!("{:?}", breakify(words, vec ));



}
