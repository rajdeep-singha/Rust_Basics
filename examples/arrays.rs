#[allow(unused)]

fn main (){
    let arr:[u32;3] = [1,2,3];
    println!("Array: {:?}", arr[0]);

    let mut arr : [u32;3] = [1,2,3];
    arr[1]=99;
    println!("Array: {:?}", arr[1]);
    
    let arr : [u32;10] = [1;10];
    println!("Array: {:?}", arr);
    // slice
    let nums : [i32;10] = [0,1,2,3,4,5,6,7,8,9];
    let slice : &[i32] = &nums[2..5];
    println!("Slice: {:?}", slice);
}