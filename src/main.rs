use std::mem;

const CURRENT_DEB:u8 = 42; //no fixed address
//se acostumbra a usar mas const que static
static STATICVAR:i32 = 123;
static mut STATICVAR2:i32 = 123;

fn data_types(){
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

    //primitive types
    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));     

    let e = 2.5; //double precision value, 8 bytes, f32 = 4 bytes, f64 = 8 bytes
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));         

    //boolean values
    let g = false; //1 byte
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));     

    let f = 4>0; //true
}

//operators
fn operators(){
    //arithmetic
    let mut a = 2+3*4;
    println!("{}", a);
    a = a+1;
    //rust no soporta 
    //los operadores incrementales ++ ni --
    a -= 2 // a = a-2
    //% es el remainder
    println!("remainder of {} / {} = {}", a, 3, (a%3));

    a_cubed =i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {} power pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise operators: only available for integers
    let c = 1 | 2; // |=OR, &=AND, ^=XOR, ! = NOT
    println!("1|2 = {}", c);

    //shift operators
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);    

    //logical: >, >=, <=, ==
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    let x = 5;
    let x_is_5 = x == 5; //true

}

fn scope_and_shadowing(){
    let a = 123;    
    //inner scope definition
    {
        let b = 456;
        println!("from inside, b = {}", b);
        let a = 777;
        println!("from inside, a = {}", a);
    }
    println!("outside a = {}", a);

}

fn main(){
    unsafe {
            println!("{}", STATICVAR2);
    }
}