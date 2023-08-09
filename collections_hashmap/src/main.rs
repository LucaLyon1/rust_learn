use std::collections::HashMap;

fn main() {
    println!("Hello, Hashmaps !");

    //Creating a new Hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 40);

    //Retrieving datas using keys :
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //Iterate over each key/value pair :
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //Hashmap & ownership :
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); //<- values are moved here
    //field_name and field_value are invalid here

    //Overwriting a value (default behavior)
    scores.insert(String::from("Blue"), 25); //<- Blue score is overwritten
    let blue_score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue score: {blue_score}");

    //Only insert if the key doesn't exist :
    scores.entry(String::from("Yellow")).or_insert(50); //<- Won't change the yellow score

    //Update the value associated to the key
    let text = "hello world wonderful world";

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);
}
