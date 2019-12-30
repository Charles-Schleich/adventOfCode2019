#![allow(warnings)]

extern crate apply;
use apply::Apply;

mod day1;
mod day2;
mod day3;
mod day4;
mod common;
mod intcode;


fn main() {
    println!("Hello, world!");
    // day1::main();
    // day2::main();
    // day3::main();    
    // day4::main();    
    // day5::main();    

    //      "ABCDE";
    // let x = "10101";
    let x = "1101";
    let hello  = &x[2..4];
    let hello2 :i32 = hello.parse().unwrap();


    // learning how to do apply
    // let y = "1101";
    // let bye:i32  = &x[2..4]
    //                .apply(|x| x.parse().unwrap()) ;
    // let bye2 :i32 = ;

 
    println!("{}",hello);
    
    // let mut y: Vec<_> = x.chars().take(3).collect();
    // println!("{:?}",y);

}
