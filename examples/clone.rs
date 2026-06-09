fn main(){
    let s1: String = String::from("I am string");
    let mut s2: String = s1.clone(); // this is very expensive
    s2.push_str(", modified");
    println!("s1 is {s1} and s2 is {s2}");
}