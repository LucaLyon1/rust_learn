fn main() {
    let mut x = 5;
    println!("The value of x is {x}");

    //The code below won't compile
    x = 6;
    println!("The value of x now is {x}");

    //Shadowing test :

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the scope is : {y}");
    }

    println!("The value of y outside is : {y}");
}
