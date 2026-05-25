use std::thread;
fn main (){
  let v: Vec<i32> = vec![1,2,3];

  let handle : thread::JoinHandle<()> = thread::spawn( move || {
      println!("Here is a vector : {:?}", v);
});
// drop(v);
handle.join().unwrap();
}