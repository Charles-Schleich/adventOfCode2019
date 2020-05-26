#![warn(unused_parens)]
use std::io::{self, BufRead};
use std::fs;
use math::round;

#[path="./common.rs"]
mod common ;
#[path="./intcode.rs"]
mod intcode ;


pub fn main(){
    let arr = common::readCSVIntoIntList("input5.txt").unwrap();
    let finalVal = part1(arr.clone());
}
//ABC DE
//010 02

fn part1(mut arr : Vec<i32>)-> i32{
    
    let input = 1;
    let finalVal = intcode::intcode(arr,input);
    println!("{:?}",finalVal);

    return 0;
}

fn part2(mut arr : Vec<i32>)-> i32{

    return 0;
}

