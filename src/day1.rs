
use std::io::{self, BufRead};
use std::fs;
use math::round;

pub fn main(){    
    
    println!("Hello");
    let contents = fs::read_to_string("input1.txt")
        .expect("Something went wrong reading the file");

    let cursor = io::Cursor::new(contents);
    let mut lines_iter = cursor.lines().map(|l| l.unwrap());
    let mut x = lines_iter.next();
    
    let mut total = 0.0;
    let mut totalModuleFuel=0.0;

    while (x!=None){
        let string = x.unwrap();
        let module : f64 = string.parse().unwrap();
        let fuel = calcmass(module);
        total=total+fuel;
        let thisExtrafuel = calcFuelRecurs(module)-module;
        totalModuleFuel=totalModuleFuel+thisExtrafuel;
        x = lines_iter.next();
    }
    println!("Total fuel {}",total);
    println!("Total Extra fuel {}",totalModuleFuel);
    
}

fn calcmass(num:f64) -> f64{
    round::floor(num/3.0,0)-2.0
}

fn calcFuelRecurs(num:f64) -> f64{
    if (num<0.0){
        return 0.0;
    }
    if (num<3.0){
        return num;
    }
    else{
        let newnum = calcmass(num);
        return num + calcFuelRecurs(newnum); 
    }
}
