#![allow(unused)]

fn add_with_return(x: u32, y: u32) -> u32 {
return x + y;
}

fn print(s: String){ 
    println!("{s}{s}{s}{s}{s}{s}{s}")
}
fn main(){
    let x=1;
    let y: u32=2;
    let z=add_with_return(x,y);
    println!("{x} + {y} = {z}");

    print("🫡".to_string());
}