// fn main() {

//     let arr: [i32 ; 4] = [11,12,13,14];
//     let mut index: usize = 0;
//     while index < 4 {
//         println!("i:{} and v:{}", index , arr[index]); // every time it checks whethere the index is less than 4 , it will check everytime making it slow and non performative 
//         index +=1;
//     }
// }

fn main() {

    let arr: [i32 ; 4] = [11,12,13,14];
    for x in arr {
        println!("x: {}", x)
    }
    
}// increases the safety of the code and eleminated the chances of index out of bounds errors