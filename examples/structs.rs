#![allow(unused)]

//struct
 #[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

struct Point3D{
    x: i32,
    y: i32,
    z: i32,
}

struct Empty;

#[derive(Debug)]
struct Circle{
    center: Point,
    radius: i32,
}

fn main (){
    let p = Point{ x:10, y:20};
    println!("Point p: ({},{})",p.x,p.y);
    let p3d = Point3D{ x:1, y:2, z:3};
    println!("Point3D p3d: ({},{},{})",p3d.x,p3d.y,p3d.z);


    let empty = Empty;
    let  circle = Circle{ center: Point{x:0,y:0}, radius: 5};
   
    println!("{:#?}", circle);

    //shortcut
    let x:i32 =1;
    let y:i32 =2;
    let p = Point {x: x, y: y};
    let p = Point {x, y}; 

    //copy feilds
    let p0 = Point {x:1,y:2};
    let p1 = Point {x:1 , y: p0.y};
    let p2 = Point{ x:2, ..p0 }; // copies remaining fields from p0
    println!("p1 copy :{:?}",p1);
    println!("p2 copy :{:?}",p2);

    // updateable structs
    let mut p = Point {x:1, y:1};
    p.x += 1;
    p.y += 2;
    println!("updated p :{:?}",p);
}

