#![allow(unused)]

use std::collections::HashMap;
fn main(){
 let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("red".to_string(), 10);
    scores.insert("blue".to_string(), 50);
    println!("scores{:?}",scores);
    
    //get value

    let score: Option<&u32> = scores.get("red");
    println!("score for red: {:?}\n", score);
    let score: Option<&u32> = scores.get("green");
    println!("score for green:{:?}",score);

    // update 
    let score: &mut u32 = scores.entry("black".to_string()).or_insert(0);
    *score += 10;

    let score: Option<&u32> = scores.get("black");
    println!("score for black: {:?}\n", score);
}