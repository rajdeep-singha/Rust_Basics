fn main (){
    let mut s1: String = String::from("Raj");
    let len = calculate_len(&mut s1);
    println!("The length of {s1} is {len}");
}

fn calculate_len(s: &mut String) -> usize { // s is a reference to a String  
    s.push_str(" Hello world");
    s.len()
}