#![allow(unused)]

// Overflow doesn't panic when compiled with --release
fn main() {
 let mut x = u32::MAX;
    x += 1;
    println!("u32 max:{}, x:{}", u32::MAX, x);

    // u32::check_add - return None on overflow
    let x =u32::checked_add(u32::MAX,1);
    println!("checked_add: {:?}", x);// :? - debug more
    // u32::wrapping_add - explicitly allow overflow
}