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

    let mut s4 = String::from("hello world");
    let word1 = first_word(&s4[..]); // convert a string to a slice
    let word2 = first_word("cargo hello"); // string literals are slices!
    println!("first words are {}, and {}", word1, word2);

    // The following `s.clear()` would result in a compiler error.
    // If it were allowed, it would invalidate the string slice at
    // `word` above.
    // s.clear(); 
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

// Note that if first_word accepted &String instead of &str,
// it would not accept string slices, and therefore would not
// accept string literals
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
