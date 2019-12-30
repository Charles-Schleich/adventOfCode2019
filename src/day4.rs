#[path="./common.rs"]
mod common;

use std::num::ParseIntError;

pub fn main(){
    let (set,count) =  part1();
    part2(set);
}

fn part1() -> (Vec<i32>,i32)  {

    let start = 353096; 
    let finish = 843212;
    let mut count = 0;
    let mut feasible = Vec::new();

    for i in start..finish{
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
            feasible.push(i);
        }
        // break;
    }
    println!("Part 1:{}",count);
    return (feasible,count)
}

fn part2 (set: Vec<i32>){

    let mut count = 0; 
    for num in set{

        let mut strNum: &str = &*num.to_string();
        let counts: Vec<_> = strNum.chars()
                                    .map(|x| strNum.matches(x).count())
                                    .collect();
        let contains2 = counts.contains(&2);
        if contains2 {
            count=count+1;
        }
    }
    println!("Part 2: {}",count);
}

fn i32toVec(num: i32) -> Vec<i32>{
  let mut s: &str = &*num.to_string();
  let split =  s.split("");
  let vecNums: Vec<i32> = split.map(|y| y.parse())
                                  .filter_map(Result::ok)
                                  .collect();
  return vecNums;
}

