
#[path="./common.rs"]
mod common;

use std::io::{self, BufRead};
use array_tool::vec::Intersect;
use std::cmp::min;

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn main(){
    let listWrapped  = common::readCSVIntoStringList("input3.txt");
    let mut list = listWrapped.unwrap();
    let (wire1Vec,wire2Vec) = createWireVecs(list);
    let intersections = part1(wire1Vec.clone(),wire2Vec.clone());
    part2(wire1Vec,wire2Vec,intersections);
}

// im not converting them to HashSets earlier is because i lose locations of repeated coordinates once i convert them 
fn part1( wire1Vec: Vec<(i32,i32)> , wire2Vec: Vec<(i32,i32)>) -> Vec<(i32,i32)>  {
    let wire1HashSet = hashset(wire1Vec);
    let wire2HashSet = hashset(wire2Vec);
    let intersections = wire1HashSet.intersection(&wire2HashSet).collect::<Vec<&(i32,i32)>>();
    let v:HashSet<(i32)> = intersections.clone().into_iter().map(|x| calcManhattan(*x)).collect();
    println!("Part 1: {:?}",v.iter().min().unwrap());
    
    let mut newIntersections =  Vec::new();
    for i in intersections{
        newIntersections.push(*i);   
    }
    return newIntersections
}


fn part2(wire1Vec: Vec<(i32,i32)> , wire2Vec: Vec<(i32,i32)>,intersections: Vec<(i32,i32)>){
    // this feels inefficient and can surely be done better
    let mut smallest = 999999;
    for i in intersections{
        let mut wire1Iter = wire1Vec.iter();
        let mut wire2Iter = wire2Vec.iter();
        let wire1Steps = wire1Iter.position(|&x| x ==  i).unwrap();
        let wire2Steps = wire2Iter.position(|&x| x ==  i).unwrap();
        let combinedSteps = wire1Steps+wire2Steps+2; // one step short for each list

        if combinedSteps < smallest{
            smallest= combinedSteps;
        }
    }
    println!("Part 2: {:?}",smallest);
    // 28578

}


//  _    _        _                      ______                    _    _                    
// | |  | |      | |                    |  ____|                  | |  (_)                   
// | |__| |  ___ | | _ __    ___  _ __  | |__  _   _  _ __    ___ | |_  _   ___   _ __   ___ 
// |  __  | / _ \| || '_ \  / _ \| '__| |  __|| | | || '_ \  / __|| __|| | / _ \ | '_ \ / __|
// | |  | ||  __/| || |_) ||  __/| |    | |   | |_| || | | || (__ | |_ | || (_) || | | |\__ \
// |_|  |_| \___||_|| .__/  \___||_|    |_|    \__,_||_| |_| \___| \__||_| \___/ |_| |_||___/
//                  | |                                                                      
//                  |_|                                                                      

fn createWireVecs(wires:Vec<Vec<String>>) -> (Vec<(i32,i32)>,Vec<(i32,i32)>) {
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

    return (wire1CoordArrayFlat,wire2CoordArrayFlat)

    // let w1HashSet = hashset(wire1CoordArrayFlat);
    // let w2HashSet = hashset(wire2CoordArrayFlat);
    // return (w1HashSet,w2HashSet)
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

fn hashset(vec: Vec<(i32,i32)>) -> HashSet<((i32,i32))> {
    HashSet::from_iter(vec)
}
