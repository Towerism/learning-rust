use std::fs::{ self, File };
use std::io::{ self, Read, ErrorKind };

fn main() {
    let f = File::open("hello.txt");

    // Create hello.txt if it doesn't exist and obtain the file handle
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e)
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error)
        }
    };

    // Alternatively we can obtain the file handle of hello.txt and panic in the
    // case of any error (including NotFound) with the unwrap helper.
    let f = File::open("hello.txt").unwrap();
    
    // We can also customize the generic error message that is displayed to
    // aid debugging later.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    let username = read_username_from_file1();
    let username1 = match username {
        Ok(u) => u,
        Err(_) => String::from("defaultuser")
    };
    let username2 = read_username_from_file2().unwrap();
    println!("The usernames read: {}, {}", username1, username2);
}

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut s = String::new();

    // If either open or read_to_string fails, we will propagate
    // the error to the calling code and return immediately.
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// read to string the "easy" way
fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
