fn main() {
    println!("Hello, collections string");

    //Creating an empty string
    let mut s = String::new();

    //Creating a String from initial data
    let data = "My initial content";

    let s_two = data.to_string();
    //other possibilities :
    //let s_two = "My initial content".to_string();
    //let s_two = String::from("My initial content");

    //appending a string to another
    let mut s_push = String::from("foo");
    s_push.push_str("bar");
    //Appending a character
    s_push.push('t');

    println!("s_push : {s_push}");
    
    //Concatenating using +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world !");
    let s3 = s1 + &s2; //s1 is moved here

    println!("s3 = {s3}");

    //Concatening many strings using the format macro
    let s_tic = String::from("tic");
    let s_tac = String::from("tac");
    let s_toe = String::from("toe");

    let s_fin = format!("{s_tic}-{s_tac}-{s_toe}");
    println!("{s_fin}");

    //iterating over strings :
    for c in "Hello world !".chars() {
        println!("{c}");
    }
    //iterating over strings' bytes
    for c in "Hello world !".bytes() {
        println!("{c}");
    }
}
