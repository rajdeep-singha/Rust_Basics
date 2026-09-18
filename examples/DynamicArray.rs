use std::marker::PhantomData;
use std::ptr::NonNull;
use std::alloc::{alloc, dealloc, Layout};
use std:mem;
struct MyVec<T>{
    ptr: NonNull<T>,
    len : usize,
    capacity : usize,

    _marker:PhantomData<T>,
}

impl<T> MyVec<T>{
  pub fn new() -> Self{
        MyVec{
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
            _marker: PhantomData,
        }
    }
    pub fn len(&self) -> usize{
        self.len
    }
    pub fn capacity(&self) -> usize{
        self.capacity
    }
    pub fn is_empty(&self) -> bool{
        self.len == 0
    }
}

fn main(){
    let  v: MyVec<i32> = MyVec::new();

    println!("len={}",v.len);
    println!("capacity = {}", v.capacity());
    println!("is_empty = {}", v.is_empty());
}