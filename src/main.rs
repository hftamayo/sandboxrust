use std::mem;

fn main(){
    println!("Hello World from Rust");
    let a:u8 = 123; //unsigned (no puede ser negativo) 8 bits == 1 byte
    println!("a = {}", a);
    //esto genera un error porque la var a es immutable a = 456;
    let mut b:i8 = 0;
    println!("b = {}", b);    
    b = 42;
    println!("b si es mutable = {}", b);        

    let mut c = 123456789; //32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));     
    c = -1;
    println!("c = {}, after modification", c);         
    //data types: i8 u8 i16 u16 i32 u32 i64 u64

    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z * 8);

}