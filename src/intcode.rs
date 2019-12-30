use std::io::{self, BufRead};
use std::fs;
use math::round;

#[path="./common.rs"]
mod common ;

fn incode(mut arr : Vec<i32>)-> i32{

    let mut exit = false;
    let mut multOf4 = 0;
    // processor in this loop
    while exit!=true{
        let mut op = arr[0 + multOf4];
        if op==99 { break; }
        let (posa, posb, posc) = ( arr[1 + multOf4] as usize
                                 , arr[2 + multOf4] as usize
                                 , arr[3 + multOf4] as usize);
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

// fn processOp(op:i32) {
fn processOp(op:i32) -> OpCode {
    let mut opStr: &str = &*op.to_string();
    let mut z = opStr.chars().rev();
    let (mut A,mut B,mut C) : (ParamMode,ParamMode,ParamMode) = (ParamMode::Pos,ParamMode::Pos,ParamMode::Pos) ;
    let mut operation = String::from("");

    for i in 0..5 {
        let character = z.next();
        if i == 0 || i == 1 {
            match character{
                Some(x) => operation.push(x),
                None =>(),
            }
        } else if i == 2 {
            A = determinParameter(character);
        } else if i == 3 {
            B = determinParameter(character);
        } else if i == 4 {
            C = determinParameter(character);
        }
    }

    let operationNum: i32 = operation.parse().unwrap();
    return OpCode{op:operationNum,var1Mode:A,var2Mode:B,var3Mode:C};
}

fn determinParameter(input:Option<char>) -> ParamMode {
    match input{
        Some(x)=> match x { 
            '0' =>  return ParamMode::Pos,
            '1' => return ParamMode::Imm,
            other => panic!()
        },
        None=> return ParamMode::Pos,
    }
}

// STRUCTS
#[derive(Debug)]
struct OpCode {
    op: i32,
    var1Mode: ParamMode,
    var2Mode: ParamMode,
    var3Mode: ParamMode
}

#[derive(Debug)]
enum ParamMode {
    Imm,
    Pos,
}





// 1002,4,3,4,33
// op par1 par2 par3


// ABCDE
//  1002

// DE - two-digit opcode,      02 == opcode 2
//  C - mode of 1st parameter,  0 == position mode
//  B - mode of 2nd parameter,  1 == immediate mode
//  A - mode of 3rd parameter,  0 == position mode,
//                                   omitted due to being a leading zero

//    0,1,2,3,4
// 1002,4,3,4,33

// 4 
