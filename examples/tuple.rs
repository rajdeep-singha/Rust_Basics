#![allow(unused)]

fn return_many() -> (u32,bool){
    (1u32,true)
}
fn main(){
    //Emplty typle = unit type
    // Result<(), String> = ok(()) | Err("💀")
let t = ();
println!("Empty tuple: {:?}", t);
    // Nested tuple
    // let nested =(('a',1.23),(true,1u32,-1i32),());
    // println!("nested.0.1:{}",(nested.0).1 )
    //Destruction a tuple
    let t:(bool,char,u32) = (true,'a',1) ;    
    let(a,b,c) = t;
    println!("a = {} , b={},c={}",a,b,c);

    // partial destruction (ignoring some values)
    let (_,b,_) = t;
    //Function that returns multiple values using a tuple
    let (a,b) = return_many();
}