fn main() {
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    takes_ownership(s3);
    // s3 was moved, ownership transferred to the scope of takes_ownership
    // trying to use s3 here would result in a compiler error
    // println!("{}", s3); // <-- compiler error
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
