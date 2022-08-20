mod my_text_lib;
use my_text_lib::{wordify, breakify};


fn main() {

    let text = String::from("where can i see my broken lines in this manually generated sentence");
    let words: Vec<String> = wordify(text);
    let line_breaks: Vec<i32> = vec![4,3,2,1];
    let result:Vec<String> = breakify(words, line_breaks );

    println!("{:?}", result);

}
