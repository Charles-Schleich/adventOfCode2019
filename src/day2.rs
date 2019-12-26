#![warn(unused_parens)]
use std::io::{self, BufRead};
use std::fs;
use math::round;

#[path="./common.rs"]
mod common ;

pub fn main(){
    let resultNumArr = common::readCSVIntoIntList("input2.txt");
    let mut arr = resultNumArr.unwrap();
    let finalVal = part1(arr.clone());
    println!("{}",finalVal);
    let finalVal1 = part2(arr);

}

fn part2(mut arr : Vec<i32>)-> i32{
    let mut noun = 0;
    let mut verb = 0;
    for i in 1..99{
        for j in 1..99{
            let mut arr2 = arr.clone();
            arr2[1]=i;
            arr2[2]=j;
            let num = part1(arr2);
            if num == 19690720 {
                println!("Done, {} {}", i, j);
                noun=i;
                verb=j;
            }
        }
    }
    println!("100*noun+verb {:?}",100*noun+verb);
    3
}

fn part1(mut arr : Vec<i32>)-> i32{

    let mut exit = false;
    let mut multOf4 = 0;

    // processor in this loop
    while exit!=true{
        let op = arr[0 + multOf4];
        if op==99 { break; }
        let (posa, posb, posc) = (arr[1 + multOf4] as usize, arr[2 + multOf4] as usize, arr[3 + multOf4] as usize);
        let (a, b) = (arr[posa], arr[posb]);
        match op {
            1=>arr[posc]=a+b,
            2=>arr[posc]=a*b,
            other=>panic!(),
        }
        multOf4=multOf4+4;
    }
    return arr[0];
}
