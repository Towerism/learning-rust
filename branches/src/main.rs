fn main() {
    let condition = true;
    let _number = if condition {
        5
    } else {
        6
    };
    println!("The value of _number is: {}", _number);
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}
