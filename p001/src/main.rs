fn main() {
    //Declare variable
    let _x = 5;
    //Variable shadowing. Changes non mutable value
    let _x = 10;
    //mutable
    let mut _y = 10;
    _y = 7;
    //Constant must be type annotated and be uppercase
    const _Z: u8 = 8;

    // Compound types
    let _tup = ("Hey", 1000);
    let (_channel, _count) = (1, 200);
    let (_xx, _yy) = _tup;

    // Arrays
    let _error_codes = [1, 2, 3];
    let _more_codes: [i32; 3] = [1, 2, 3];
    let _element = _more_codes[0];

    let _ret = my_function(55);

    // Conditionals. Traditional if else statements
    // Conditional statments can also be use in varibel
    let cond = 
        if 4 > 3 { 9 } else { 10 };
    println!("{}", cond);

    // three types of loops: loops, for, while
    // loop: must call break, can return values
    let mut cnt = 0;
    let loopres = loop {
        println!("round {}", cnt);
        cnt += 1;
        if cnt == 5 { break cnt; }
    };
    println!("Loop return {}", loopres);
    //While
    while cnt != 0 {
        println!("While loop round {}", cnt);
        cnt -= 1;
    }
    // For loop
    for e in _error_codes.iter() {
        println!("{}", e);
    }
    //range based loops. 1 to 3
    for i in 1..4 {
        println!("{}", i);
    }
}

// Functions
fn my_function(arg: i32) -> i32
{
    println!("func {}", arg);
    //returns. Return keyword can be used
    arg * 2
}
