#![allow(dead_code)]

use std::mem;

const MEANING_OF_LIFE: u8 = 42; // no fixed address

static mut Z: i32 = 123;

fn operators() {
    let mut a = 2 + 3 * 3;
    println!("{}", a);

    a = a + 1;
    a -= 2;

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let PI = std::f64::consts::PI;

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, PI);
    println!("{} cubed is {}, {}^PI IS {}", b, b_cubed, b, b_to_pi);

    let c = 1 | 2;

    println!("1|2 = {}", c);

    let two_to_ten = 1 << 10; // >>
    println!("2^10 = {}", two_to_ten);

    // logical
    let pi_less_4 = PI < 4.0;
}

fn scope_and_shadowing() {
    let a = 123;
    let a = 1234;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    // println!("outside, b = {}", b);
}

fn fundamental_data_types() {
    let a: u8 = 123;
    println!("a = {}", a);

    let mut b: i8 = 0;

    println!("b = {}", b);

    b = 42;

    println!("b = {}", b);

    let mut c = 123456789;

    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!(
        "c = {} after modification, size = {} bytes",
        c,
        mem::size_of_val(&c)
    );

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);

    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';

    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e: f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;

    println!("{}", p3.x);
}

fn main() {
    // operators();
    // scope_and_shadowing();
    // println!("{}", MEANING_OF_LIFE);

    unsafe {
        println!("{}", Z);
    }

    stack_and_heap();
}
