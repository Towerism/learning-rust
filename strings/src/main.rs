fn main() {
    let s1 = "नमस्ते";
    let first_char = &s1[0..=2]; // unicode!
    println!("first_char: {}", first_char);
    println!("s1 characters:");
    for c in s1.chars() {
        println!("{}", c);
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved and is no longer a valid reference

    println!("{}", s3)
}
