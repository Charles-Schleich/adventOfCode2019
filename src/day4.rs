#[path="./common.rs"]
mod common;

use std::num::ParseIntError;

pub fn main(){
    part1();
}

fn part1() {

    let start = 353096; 
    let finish = 843212;
    let mut count = 0;

    for i in 353096..843212{
        let vecNums = i32toVec(i);
        
        let mut increase = true;
        let mut adjacent = false;
        for i in 1..6 {
            if vecNums[i]< vecNums[i-1]{
                increase=false;
                break;
            }
            if vecNums[i] == vecNums[i-1]{
                adjacent=true;
            }
        }
        if adjacent==true && increase == true {
            count = count+1;
        }
        // break;
    }
    println!("Part 1:{}",count);
}

fn part2(){
                         
}




fn i32toVec(num: i32) -> Vec<i32>{
  let mut s: &str = &*num.to_string();
  let split =  s.split("");
  let vecNums: Vec<i32> = split.map(|y| y.parse())
                                  .filter_map(Result::ok)
                                  .collect();
  return vecNums;
}

