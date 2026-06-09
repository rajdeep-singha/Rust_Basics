fn main() {
    let num =10;
    let result = add(num);

    let name = String::from("Rust language");
    takes_ownership(name);

    let s : String = gives_ownership();
     println!("Num is {num} and result is {result}");
     // println!("value of name is {name}")
     println!("value of s is {s}");
}

fn takes_ownership(s: String) {
    println!("Inside ownership {s}");
}

fn gives_ownership() -> String{
    let s = String::from("This is a string ownership");
    s
}

fn add(x:i32)->i32{
    x+10
}