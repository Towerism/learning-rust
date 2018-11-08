fn main() {
    let s1 = gives_ownership();
    let s1_len = calculate_length(&s1); // does not take ownership of s1
    println!("length of {} (s1): {}", s1, s1_len); // s1 can still be used!
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
    // &some_string would result in a dangling pointer, and the compiler would complain!
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // because we took ownership, some_string is deallocated once it goes out of scope

// calculate length of input string without taking ownership of it
fn calculate_length(s: &String) -> usize {
    s.len()
}
