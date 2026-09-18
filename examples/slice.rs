fn main(){
    let s1 = "Hello";
    find_first_word(s1);
    let mut s = String::from("hello world");  // fn takes a string of words separated by spaces and returns the 1stspace of words if no space is found then return the string 
    let res = find_first_word(&s);
    

    println!("For String {s} Result is {}",res.len());
    s.clear();
} 

fn find_first_word(input:&str) -> &str { // &str instead of &string 
 let s = input.as_bytes();
 for(i, &item) in s.iter().enumerate(){
    if item == b' '{
        return &input[..i];
 }
}
&input[..s.len()]
}




// fn main(){
//  let s = String::from("hello world");

//  let hello = &s[0..5];
//  let world = &s[6..11];

//  println!("Hello = {hello}");
//  println!("World = {world}");

// }
// fn find_first_word(input:&String) -> usize {
//  let s = input.as_bytes();
//  for(i, &item) in s.iter().enumerate(){
//     if item == b' '{
//         return i;
//  }
// }
// s.len()
// }