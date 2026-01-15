#![allow(unused)]

use std::vec;

//vector
fn main(){
    let mut v : Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector v: {:?}",v);

    let v: Vec<i8> = vec![1,2,3];
    let v = vec![1u8,2,3];

    let v: Vec<i8> = vec![0i8;100]; //100 elements initialized to 0
    println!("Vector v with 100 elements: {:?}",v);

    //get elements
    println!("v: {}",v[10]);

    // option<&i8>
    // index valid => some(&value)
    // index invalid => none
    println!("v[1]: {:?}", v.get(1));
    println!("v[200]: {:?}", v.get(200));

    //pop - remove last elements 
    let mut v: Vec<i8> = vec![1,2,3];
// 3
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x);
    // 2
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x);
    // 1
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x);
    // none
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x);


    // slice
    let v = vec![1,2,3,4,5];
    let s = &v[1..4]; // slice from index 1 to 3
    println!("slice s: {:?}", s);

}