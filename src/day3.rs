
#[path="./common.rs"]
mod common;

use std::io::{self, BufRead};
use array_tool::vec::Intersect;
use std::cmp::min;

pub fn main(){
    let listWrapped  = common::readCSVIntoStringList("input3.txt");
    let mut list = listWrapped.unwrap();
    part1(list);
}

fn part1(wires:Vec<Vec<String>>){
    let wire1 = wires[0].to_owned();
    let wire2 = wires[1].to_owned();
    let mut curCoords = (0,0);
    let mut wire1CoordArray = Vec::new();
    let mut wire2CoordArray = Vec::new();    

    // for wire1
    for i in wire1 {
        let newVec = handleDirection(curCoords, &i);
        curCoords = *newVec.last().unwrap();
        wire1CoordArray.push(newVec);
    };
    
    // for wire2
    let mut curCoords = (0,0);
    for i in wire2 {
        let newVec = handleDirection(curCoords, &i);
        curCoords = *newVec.last().unwrap();
        wire2CoordArray.push(newVec);
    };
    
    let wire1CoordArrayFlat : Vec<(i32,i32)> = wire1CoordArray.into_iter().flatten().collect();
    let wire2CoordArrayFlat : Vec<(i32,i32)> = wire2CoordArray.into_iter().flatten().collect();

    println!("Hello");

    let intersections = wire2CoordArrayFlat.intersect(wire1CoordArrayFlat);
    println!("{:?}",intersections);
    
    let v: Vec<i32> = intersections.into_iter().map(|x| calcManhattan(x)).rev().collect();

    println!("{:?}",v.iter().min().unwrap());

    println!("done");
}

fn handleDirection(CurrentCoords:(i32,i32), wire:&str) -> Vec<(i32,i32)> {
    let mut newVec = Vec::new();
    let direction = &wire[0..1];
    let jump :i32 = wire[1..].parse().unwrap();
    let (x,y) = CurrentCoords; 

    match direction {
        "U" =>  for i in 1..jump+1 { newVec.push((x,y+i)); } ,
        "D" => for i in 1..jump+1 { newVec.push((x,y-i)); } ,
        "L" => for i in 1..jump+1 { newVec.push((x-i,y)); } ,
        "R" => for i in 1..jump+1 { newVec.push((x+i,y)); } ,
        &_ =>panic!("Fail")
    };
    return newVec
}

fn calcManhattan(point:(i32,i32)) -> i32 {
    let (x,y) = point;
    return x.abs()+y.abs();
}