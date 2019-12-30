use std::io::{self, BufRead};
use std::fs;
use math::round;

#[path="./common.rs"]
mod common ;

pub fn intcode(mut arr : Vec<i32>)-> i32{

    let mut exit = false;
    let mut multOf4 = 0;
    // processor in this loop
    while exit!=true{
        let mut op = arr[0 + multOf4];
        if op==99 {
            println!("99 Exit found"); 
            break; 
        }
        let opMode = processOp(op);

        let (arga, argb, argc) = ( arr[1 + multOf4] 
                                 , arr[2 + multOf4] 
                                 , arr[3 + multOf4] );
        let args = Args{argA:arga ,argB:argb ,argC:argc};
        println!("Here {:?}",arr );
        match opMode.op {
            1=>arr = op1(arr,opMode,args),
            2=>arr = op2(arr,opMode,args),
            other=>panic!(),
        }
        println!("Here {:?}",arr );
        multOf4=multOf4+4;
    }
    return arr[0];
}


//    ____   _____    _____ 
//   / __ \ |  __ \  / ____|
//  | |  | || |__) || (___  
//  | |  | ||  ___/  \___ \ 
//  | |__| || |      ____) |
//   \____/ |_|     |_____/ 
                      
// arr[posc]=a+b,
fn op1(mut arr : Vec<i32>, mode:OpCodeMode, args:Args ) -> Vec<i32>{
    let a = getArgValue(&arr, mode.var1Mode, args.argA );
    let b = getArgValue(&arr, mode.var2Mode, args.argB );
    let c = getArgValue(&arr, mode.var3Mode, args.argC );
    arr[c as usize]=a+b;
    return arr;
}

// arr[posc]=a*b,
fn op2(mut arr : Vec<i32>, mode:OpCodeMode, args:Args ) -> Vec<i32>{
    // println!("Arr: {:?} \n mode {:?} \n args {:?}",arr,mode,args);
    let a = getArgValue(&arr, mode.var1Mode, args.argA );
    let b = getArgValue(&arr, mode.var2Mode, args.argB );
    let c = args.argC ; // write operation
    // println!("a {:?}, b {:?}, c{:?} ",a,b,c );
    arr[c as usize]=a*b;
    return arr;
}

//   _    _  ______  _       _____   ______  _____    _____  
//  | |  | ||  ____|| |     |  __ \ |  ____||  __ \  / ____| 
//  | |__| || |__   | |     | |__) || |__   | |__) || (___   
//  |  __  ||  __|  | |     |  ___/ |  __|  |  _  /  \___ \  
//  | |  | || |____ | |____ | |     | |____ | | \ \  ____) | 
//  |_|  |_||______||______||_|     |______||_|  \_\|_____/  

fn processOp(op:i32) -> OpCodeMode {
    let mut opStr: &str = &*op.to_string();
    let mut z = opStr.chars().rev();
    let (mut A,mut B,mut C) : (ParamMode,ParamMode,ParamMode) = (ParamMode::Pos,ParamMode::Pos,ParamMode::Pos) ;
    let mut opA = String::from("");
    let mut opB='0';
    
    for i in 0..5 {
        let character = z.next();
        if i == 0 {
            match character{
                Some(x) => opB=x,
                None =>(),
            }
        }
        if i == 1 {
            match character{
                Some(x) => opA.push(x),
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
    opA.push(opB);
    let operationNum: i32 = opA.parse().unwrap();
    return OpCodeMode{op:operationNum,var1Mode:A,var2Mode:B,var3Mode:C};
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

fn getArgValue(arr : &Vec<i32>, mode: ParamMode, arg:i32 ) -> i32 {
    match mode {
        ParamMode::Imm => return arg,              // Returns value raw
        ParamMode::Pos => return arr[arg as usize] // Returns value at that position
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
    var1Mode: ParamMode,
    var2Mode: ParamMode,
    var3Mode: ParamMode
}

#[derive(Debug)]
struct Args {
    argA: i32,
    argB: i32,
    argC: i32,
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
