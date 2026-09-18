use std::marker::PhantomData;
use std::ptr::NonNull;
use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ops::Index;

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

  fn grow(&mut self){
    let new_capacity = if self.capacity == 0{
        1
    }
    else{
        self.capacity * 2
    };

    let new_layout = 
        Layout::array::<T>(new_capacity).expect("Layout error");

    let new_ptr = unsafe {
        alloc(new_layout) as *mut T  // ALLOC RETURN  *MUT U8  but my vector stores ptr:NonNull<T>
    };

    let new_ptr = 
        NonNull::new(new_ptr).expect("Failed to allocate memory");
    if self.capacity > 0{
        let old_layout = Layout::array::<T>(self.capacity).expect("Layout error");
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.ptr.as_ptr(),
                new_ptr.as_ptr(),
                self.len,
            );
            dealloc(
                self.ptr.as_ptr() as *mut u8,
                    old_layout,
            );
        }
    }
    self.ptr = new_ptr;
    self.capacity = new_capacity;
  }

  pub fn push(&mut self, value: T) {
    if self.len == self.capacity {
        self.grow();
    }

    unsafe {
        self.ptr
            .as_ptr()
            .add(self.len)
            .write(value);
    }

    self.len += 1;
}

  pub fn pop(&mut self)-> Option <T>{
    if self.len == 0 {
    return None;
 }
 self.len -=1;

 unsafe{
    Some(self.ptr.as_ptr().add(self.len).read())
 }
}
// Returns a reference to the element at the given index, if it exists.
pub fn get(&self, index: usize) -> Option<&T> {
    if index >= self.len {
        return None;
    }

    unsafe {
        Some(&*self.ptr.as_ptr().add(index))
    }
}

pub fn iter(&self) -> Iter<'_, T> {
    unsafe {
        Iter {
            ptr: self.ptr.as_ptr(),
            end: self.ptr.as_ptr().add(self.len),
            _marker: PhantomData,
        }
    }
}

}

impl<T> Drop for MyVec<T>{
    fn drop(&mut self){
        unsafe{
            for i in 0..self.len{
                std::ptr::drop_in_place(
                    self.ptr.as_ptr().add(i)
                );
            }
                if self.capacity !=0{
                    let layout = Layout::array::<T>(self.capacity)
                        .expect("Layout error");

                    dealloc (
                        self.ptr.as_ptr() as *mut u8,
                        layout,
                    );
                }
            }
        }
    }


pub struct Iter<'a, T> {
    ptr: *const T,
    end: *const T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }

        unsafe {
            let item = Some(&*self.ptr);
            self.ptr = self.ptr.add(1);

            (item)
        }
    }
}

impl<T> Index<usize> for MyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
            .expect("index out of bounds")
    }
}

fn main(){
    let mut v =  MyVec::new();

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);

    println!("len = {}" , v.len());
    println!("capacity= {}", v.capacity());
    
    println!("v[0] = {}", v[0]);
    println!("v[1] = {}", v[1]);
    
    println!("Iterating:");

    for value in v.iter() {
        println!("{}", value);
    }

    println!("pop = {:?}", v.pop());
    println!("pop = {:?}", v.pop());
}