fn main() {
    // call_me(8, 'h');

    let result = return_num(5); 

    println!("The value is: {}", result);
}

fn call_me(num: u8, name: char) {
    println!("The supplied arg is: {}, {}", num, name);
}

// functions that return something uses the -> in the function definition and dropping the semicolon implies a return
fn return_num(num: i32) -> i32 {
    5 * num
}