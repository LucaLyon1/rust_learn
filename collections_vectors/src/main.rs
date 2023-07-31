fn main() {
    println!("Hello, world!");

    //We create a vector of i32
    let v: Vec<i32> = Vec::new();

    //We create a vector of i32 using the vec! macro and type inference
    let v_macro = vec![1,2,3];

    //We create a mutable vector
    let mut v_push = Vec::new();

    //and push i32 values in it
    v_push.push(1);
    v_push.push(2);
    v_push.push(3);

    //We read the third element of the vector using square brackets
    let third: &i32 = &v_push[2];
    println!("The third element is {third}");

    //We read the third element of the vector using get method
    let third_get: Option<&i32> = v_push.get(2);
    match third_get {
        Some(third_get) => println!("The third element using get is {third}"),
        None => println!("There is no third element"),
    }

    //Looping over all the elements of a vector
    let v_new =  vec![100,50,25,12,6,3,2,1,0];
    for i in &v_new {
        println!("{i}");
    }

    //Looping & modifying the elements of a mutable vector
    let mut v_mut = vec![0,1,2,3,4,5,6];
    for i in &mut v_mut {
        *i += 1;
        println!("{i}");
    }

    //Cheating with enums to have multiple types in vector
    enum Columns {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Columns::Int(3),
        Columns::Float(15.5),
        Columns::Text(String::from("Viva la vida")),
    ];
}
