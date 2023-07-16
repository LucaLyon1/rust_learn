fn main() {
    println!("Hello, world!");

    another_function(10, 'm');

    let sum = return_function(5,4);

    println!("The value of the sum is {sum}");

    //loop_labels(5);

    for_loop([1,2,3,4,5]);
}

fn another_function(x: i32, unit_label: char) {
    println!("The value you measured is : {x}{unit_label}");
}

fn return_function(x: i32, y: i32) -> i32 {
    x+y
}

fn loop_labels(count: i32) {
    let mut tmp = 0;
    'counting: loop {
        println!("count = {tmp}");
        let mut range = 0;
        loop {
            if range == tmp {
                break;
            }
            println!("Range = {range}");
            if tmp == count {
                break 'counting;
            }
            range += 1;
        }
        tmp += 1;
    }
}

fn for_loop(array: [i32; 5]) {
    for element in array {
        println!("{element}")
    }
}