

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



fn processOp(op:i32) -> OpCode{
    let mut opStr: &str = &*op.to_string();
    let len = opStr.len();
    match len {
        // 1 digit,  ABCDE: ABC = position mode
        1=> return OpCode{ op:op
                         , var1Mode:ParamMode::Pos
                         , var2Mode:ParamMode::Pos
                         , var3Mode:ParamMode::Pos } , 
        // 2 digits, ABCDE: ABC = position mode
        2=> return OpCode{ op:op
                         , var1Mode:ParamMode::Pos
                         , var2Mode:ParamMode::Pos
                         , var3Mode:ParamMode::Pos },   
        // 3 digits, ABCDE: AB  = position mode,  C = Immediate mode
        3=> {
            let onlyOpStr  = &opStr[1..3];
            let onlyOpInt :i32 = onlyOpStr.parse().unwrap();

            return OpCode{ op:onlyOpInt
                         , var1Mode:ParamMode::Pos
                         , var2Mode:ParamMode::Pos
                         , var3Mode:ParamMode::Imm};                
        },
        // 4 digits, ABCDE: A   = position mode,  B = Immediate mode, C Either
        4=>{
            
            // if opChars[] {
            // }
            return OpCode{op:op
                         ,var1Mode:ParamMode::Pos
                         ,var2Mode:ParamMode::Imm
                         ,var3Mode:ParamMode::Imm} //????
        },                
        // 4 digits, ABCDE: A   = Immediate mode, CB Either
        5=>{

            return OpCode{op:op
                         ,var1Mode:ParamMode::Imm
                         ,var2Mode:ParamMode::Imm    //????
                         ,var3Mode:ParamMode::Imm}   //????
        },                
        other=>panic!(),
    }
}

// STRUCTS
struct OpCode {
    op: i32,
    var1Mode: ParamMode,
    var2Mode: ParamMode,
    var3Mode: ParamMode
}

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
