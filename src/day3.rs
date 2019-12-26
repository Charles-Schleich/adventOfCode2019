#![warn(unused_parens)]
#![allow(non_snake_case)]
#![warn(unused_parens)]

use std::io::{self, BufRead};
use std::fs;
use math::round;

#[path="./common.rs"]
mod common ;

pub fn main(){
    let listWrapped  = common::readCSVIntoStringList("input3.txt");
    let mut list = listWrapped.unwrap();
    // println!("{:?}",list);
    part1(list);
}

fn part1(wires:Vec<Vec<String>>){
    let wire1 = wires[0].to_owned();
    let mut coordinates=0;
    
    let mut curCoords = (0,0);  
    println!("Begin");
    for i in wire1{
        let newVec = handleDirection(curCoords, &i);
    }
    println!("done");
}


fn handleDirection(CurrentCoords:(i32,i32), wire:&str) -> Vec<i32> {
    let mut newVec = Vec::new();

    let direction = &wire[0..1];
    let jump :i32 = wire[1..].parse().unwrap() ;
    // [(),()]
    // let mut a = Vec::with_capacity(jump);
    // let mut vec = Vec::new();
    
    match direction {
        "U" => println!("Up"),
        "D" => println!("Down"),
        "L" => println!("Left"),
        "R" => println!("Right"),
        &_ =>panic!("Fail")
    };

    // for i in jump{

    // };
    
    // println!("{:?}",a[12]);
    newVec.push(12);
    return newVec
}
