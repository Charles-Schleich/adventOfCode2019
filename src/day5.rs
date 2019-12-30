#![warn(unused_parens)]
use std::io::{self, BufRead};
use std::fs;
use math::round;

#[path="./common.rs"]
mod common ;
#[path="./intcode.rs"]
mod intcode ;


pub fn main(){
    let resultNumArr = common::readCSVIntoIntList("input2.txt");
    let mut arr = resultNumArr.unwrap();
    let finalVal = part1(arr.clone());

}

fn part1(mut arr : Vec<i32>)-> i32{
    intcode

}

fn part2(mut arr : Vec<i32>)-> i32{

}

