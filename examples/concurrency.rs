use std::{thread,time::Duration};
use std::thread::JoinHandle;
fn main (){
let handle:JoinHandle<()> = thread::spawn(|| {
   for i in 1..10{
    println!("hi number {} from the spawned thread!",i);
    thread::sleep(Duration::from_millis(1));
   }
});
handle.join().unwrap(); // join button can control whether my 

   for i in 1..5{
    println!("hi number {} from the main thread!",i);
    thread::sleep(Duration::from_millis(1));
}

}