use std::io;

fn main() {
    // Scalar types - represents a single value. They are: numbers, floating-point numbers, booleans and characters
    
    // *** INTEGERS *** - A number without a decimal

    // signed ints start with i
    // unsigned ints start with u

    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	isize	    usize

    // when the sign matters, ie a number can be nagative the type must be signed - conversely if it's safe to assume that the number is always positive, unsigned will suffice
    // signed i8 = -128 to 127
    // unsigned i8 = 0 to 255.

    // *** FLOATING POINT NUMBERS *** - A number with a decimal

    let floating_num = 2.0; // <- f64
    let floating_num: f32 = 2.0; // <- f64

    // Rust supports basic math operations ie: + - / *

    // *** BOOLEANS *** - nothing new here

    // *** CHARACTERS *** - basically strings with single quote

    // *** COMPOUNDS *** - groups multiple values into one type

    // Tuples - Have a fixed length - they store values like an array

    let tup = (500, 4.2, 1);

    // access values by destructuring
    let (x, y, z) = tup;

    println!("The value of y is: {}, {}, {}", x, y, z);

    // or access them directly using indeces
    println!("The first value is: {}, {}, {}", tup.0, tup.1, tup.2);

    // Arrays - usefull for fixed size lists otherwise Vectors are the way to go
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // <- 5 threes

    let first_index = a[3];

    println!("Third index: {}", first_index);

    let a = [1, 2, 3, 4, 5];

    println!("Enter an index!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Error reading user input");

    let index: usize = index
        .trim()
        .parse()
        .expect("Error parsing string");
    
    let element = a[index];

    println!("The values are {} and {}", index, element);

}
