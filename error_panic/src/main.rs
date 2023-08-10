fn main() {
    println!("Hello, panic!");

    panic!("The code will panic !");

    let v = vec![1,2,3];

    v[99]; //<- The code will panic
}
