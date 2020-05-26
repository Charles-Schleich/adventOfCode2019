use std::io::{self, BufRead};
use std::fs;
use math::round;

#[path="./common.rs"]
mod common ;

pub fn intcode(mut arr : Vec<i32>, input: i32)-> i32{
    println!("INT CODE START\n");
    let mut exit = false;
    let mut arrOffset = 0;
    
    // processor in this loop
    while exit!=true{
        
        let mut op = arr[0 + arrOffset];
        let mut opMode = processOp(op);
    
        if opMode.op == 99 {
            println!("99 found: Exit"); 
            break; 
        }
        
        opMode = getparams(&mut arr,arrOffset , opMode); 
        arr = match opMode.op{
            1 => { 
                    arrOffset = arrOffset + 4;
                    op1(arr,opMode)
                },
            2 => { 
                    arrOffset = arrOffset + 4;
                    op2(arr,opMode)
                 },
            3 => { 
                    arrOffset = arrOffset + 2;
                    op3(arr,opMode, input)
                 },
            4 => { 
                    arrOffset = arrOffset + 2;
                    op4(arr,opMode)
                 },
            _ => {unimplemented!()}
        };
    }
    return arr[0];
}


fn getparams( arr : &mut Vec<i32>, arrOffset:usize,  mode: OpCodeMode) -> OpCodeMode{
    let mut params :Vec<i32> = Vec::new();
    for i in 1..mode.numArgs+1 {
        let param = arr[arrOffset + i as usize];
        params.push(param)
    }
    return OpCodeMode{ op: mode.op
                     , params:params
                     , modes:mode.modes
                     , numArgs:mode.numArgs
                     };
}


//    ____   _____    _____ 
//   / __ \ |  __ \  / ____|
//  | |  | || |__) || (___  
//  | |  | ||  ___/  \___ \ 
//  | |__| || |      ____) |
//   \____/ |_|     |_____/ 

// arr[posc]=a+b,
fn op1(mut arr : Vec<i32>, ocm:OpCodeMode ) -> Vec<i32>{
    let a = getArgValue(&arr, ocm.modes[0], ocm.params[0] );
    let b = getArgValue(&arr, ocm.modes[1], ocm.params[1] );
    let c = ocm.params[2];
    // let c = getArgValue(&arr, ocm.modes[2], ocm.params[2] );

    arr[c as usize]=a+b;
    return arr;
}


// arr[posc]=a*b,
fn op2(mut arr : Vec<i32>, ocm:OpCodeMode ) -> Vec<i32>{
    let a = getArgValue(&arr, ocm.modes[0], ocm.params[0] );
    let b = getArgValue(&arr, ocm.modes[1], ocm.params[1] );
    // let c = getArgValue(&arr, ocm.modes[2], ocm.params[2] );
    let c = ocm.params[2];
    arr[c as usize]=a*b;
    return arr;
}

fn op3(mut arr : Vec<i32>, ocm:OpCodeMode, input:i32 ) -> Vec<i32>{
    let a = ocm.params[0] as usize;
    arr[a]=input;
    return arr;
}

fn op4(mut arr : Vec<i32>, ocm:OpCodeMode) -> Vec<i32>{
    let a = getArgValue(&arr, ocm.modes[0], ocm.params[0] );
    // let a = ocm.params[0] as usize;
    println!("{}",a);
    return arr;
}



//   _    _  ______  _       _____   ______  _____    _____  
//  | |  | ||  ____|| |     |  __ \ |  ____||  __ \  / ____| 
//  | |__| || |__   | |     | |__) || |__   | |__) || (___   
//  |  __  ||  __|  | |     |  ___/ |  __|  |  _  /  \___ \  
//  | |  | || |____ | |____ | |     | |____ | | \ \  ____) | 
//  |_|  |_||______||______||_|     |______||_|  \_\|_____/  

fn processOp(op:i32) -> OpCodeMode {
    let mut opStr: &str = &*op.to_string();         // 1002
    let mut z = opStr.chars().rev();    // 20010
    let units = z.next().unwrap();
    let tens = match z.next() {
        Some(ch) =>{ch},
        None =>{'0'}
    };
    let mut opcode = String::new();
    opcode.push(tens);
    opcode.push(units);
    let opcodeNum = opcode.parse().unwrap();
    let numArgs = getNumOfArgs(opcodeNum);
    let mut modes: Vec<ParamMode>= Vec::new();

    for i in 0..numArgs{
        let paramMode = match z.next() {
            Some(ch) =>{determineParamMode(ch)},
            None           =>{determineParamMode('0')}
        };
        modes.push(paramMode);
    }

    return OpCodeMode{ op:opcodeNum
                     , params:Vec::new()
                     , modes:modes
                     , numArgs:numArgs};
}


fn determineParamMode(input:char) -> ParamMode {
    match input{
        '0' =>  return ParamMode::Pos,
        '1' => return ParamMode::Imm,
        other => panic!()
        }
}

fn getArgValue(arr : &Vec<i32>, mode: ParamMode, arg:i32 ) -> i32 {
    match mode {
        ParamMode::Imm => return arg,              // Returns value raw
        ParamMode::Pos => return arr[arg as usize] // Returns value at that position
    }
}

fn getNumOfArgs(op: i32) -> i32{
    match op{
        99 =>{0}
        1 | 2 =>{3}
        3 | 4 =>{1}
        _ =>{ unimplemented!()}
    }
}






//    _____  _______  _____   _    _   _____  _______  _____ 
//   / ____||__   __||  __ \ | |  | | / ____||__   __|/ ____|
//  | (___     | |   | |__) || |  | || |        | |  | (___  
//   \___ \    | |   |  _  / | |  | || |        | |   \___ \ 
//   ____) |   | |   | | \ \ | |__| || |____    | |   ____) |
//  |_____/    |_|   |_|  \_\ \____/  \_____|   |_|  |_____/ 
                                                        
#[derive(Debug)]
struct OpCodeMode {
    op: i32,
    modes: Vec<ParamMode>,
    params: Vec<i32>,
    numArgs: i32
}

#[derive(Debug)]
struct Args {
    argA: i32,
    argB: i32,
    argC: i32,
}

#[derive(Debug,Copy,Clone)]
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
