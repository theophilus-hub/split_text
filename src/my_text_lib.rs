
// spliting your text into words stored in a vector 
pub fn wordify(text: String) -> Vec<String> {

    let mut result: Vec<String> = Vec::new();
    
    for i in text.split_whitespace(){
        result.push(i.to_string());
    }
    
    result
}


//grouping the words for each line based on the recieved number of words per line
pub fn breakify(words: Vec<String>, words_per_line: Vec<i32>) -> Vec<String> {

    let mut result: Vec<String> = Vec::new();
    let mut words_per_line_checker: i32 = 0;
    let  remainder: i32;
    let mut last_break = 0;


    //checking for number of words and possible remainder of words per line
   for i in words_per_line.clone(){
        words_per_line_checker = words_per_line_checker+ i;
   }
   
   if words_per_line_checker <= words.len() as i32{
        remainder = words.len() as i32 - words_per_line_checker;

        for i in words_per_line.clone(){
            let mut phrase: String = String::new();

        for j in 0..i as usize{
            phrase.push_str(&format!("{}",words[last_break + j]));
            phrase.push_str(" ");
        }
        
        last_break = last_break + i as usize;
        result.push(phrase);
    }
    
    //adding the remaining words in the text
    if remainder > 0{
        let mut phrase: String = String::new();

        for i in 0..remainder as usize{
            phrase.push_str(&format!("{}",words[words_per_line_checker as usize + i]));
            phrase.push_str(" ")
        }
        result.push(phrase);
        
    }

   }else{
    println!("number of words per line exceeds the total number of words in your text")
   }



   //println!("{}", remainder);
    result
}