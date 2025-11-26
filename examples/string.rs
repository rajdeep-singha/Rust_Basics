#![allow(unused)]

fn main(){
    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    // Concatenate strings using the + operator
    let s3: String = s1 + &s2; // Note: s1 is moved here and can no longer be used
    println!("Concatenated String: {}", s3);

    // Using the format! macro to concatenate strings without taking ownership
    let s4: String = String::from("Hello, ");
    let s5: String = String::from("Rustaceans!");
    let s6: String = format!("{}{}", s4, s5);
    println!("Formatted String: {}", s6);

    // Slicing a string
    let hello: String = String::from("Hello, world!");
    let slice: &str = &hello[0..5]; // Slices the first 5 characters
    println!("String Slice: {}", slice);

    // Rust automatically converts &String to &str when needed
    let msg: String  = String::from("Hello, Rust!");
    print(&msg);
    
    let s:&str = "Hello World";
    print(s);
    // Appending a string slice to a String
    let mut msg: String = String::from("Hello Rust");
    msg.push_str("oceans!");
    println!("{msg}");


    // string interpolation - format !
    let lang = "Rust";
    let emoji = "🦀";
    let s = "Hello Rust !";
    let s = format!("Hello {} {}", lang, emoji);
    println!("{s}");

} fn print(s: &str) {
        println!("String slice in function: {}", s);
    }
    