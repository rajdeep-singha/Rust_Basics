use std:: sync::mpsc;
use std::thread::{self, spawn};
fn main(){
let (tx, rx) = mpsc::channel(); 
    thread::spawn(||{
          let msg: String = String::from("Hello, world!");
          tx.send(msg).unwrap();
    });

}