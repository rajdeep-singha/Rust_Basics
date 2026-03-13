#[allow(unused)]
fn main(){
    let mut i=0;
    loop{
        println!("loop");
        i += 1;
        if i == 10 {
            break;
        }
    }
    for i in 0..6 {
        println!("for loop {i}");
    }
    
    let arr = [1,2,3,4,5];
    let n : usize = arr.len();
    for i in 0..n {
        println!("arr[{i}] = {}", arr[i]);
    }
}
