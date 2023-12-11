
fn main() {
    let function_name = "println!";
    // prefix with _ if not used
    let _function = match function_name {
        "println!" => println!("Hello, world!"),
        _ => println!("Function not found!"),
    };
}
