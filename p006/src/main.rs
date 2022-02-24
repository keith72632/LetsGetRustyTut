// ---- Error Handling
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // kills program and prints out errors
    // back tracing prints out info related to error set env var RUST_BACKTRACE=1(export on linux)
    c(7);

    /*
    Result enum return success or failure
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    // let f = File::open("hello.txt"); // file could or could not be there
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         //create new file
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         }
    //         other_error => {
    //             panic!("Problem opening the other file: {:?}", other_error)
    //         }
    //     },
    // };

    // let f2 = File::open("notes.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("notes.txt").unwrap_or_else(|error|{
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file {:?}", error);
    //     }
    // });

    // a normal unrwap will return a result enum
    // let f3 = File::open("another.txt").unwrap();
    //or 
    // let f4 = File::open("another3.txt").expect("Failed to open another3.txt");

    // in rust, it's custom to return error in function to caller
    let s = read_username_from_file();

    println!("{:?}", s);
}

fn c(num: i32) {
    if num == 6 { panic!("crash and burn"); };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // stores result type in f

    // if no ? on the end of Result type, then use below
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    // read to string also returns result type
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    f.read_to_string(&mut s)?;
    Ok(s)
}