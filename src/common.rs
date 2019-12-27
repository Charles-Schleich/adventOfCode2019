use std::error::Error;

use std::fs;
use std::io;
use std::process;
use std::fs::File;

// refactor this to parse column of thing 
pub fn readFileIntoList(filename: &str){
    // let contents = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");

    // let cursor = io::Cursor::new(contents);
    // let mut lines_iter = cursor.lines().map(|l| l.unwrap());
    // let mut x = lines_iter.next();

    // // let mut numlist:[i32] = []; 
    // let mut vec = Vec::new();

    // while (x!=None){
    //     let string = x.unwrap();
    //     let number : i32 = string.parse().unwrap();
    //     vec.push(number);
    //     x = lines_iter.next();
    //     println!("Number {} ",number);
    // }
    // println!("Vector {:?} ",vec);
}

// Todo: find better way to deserilize
pub fn readCSVIntoIntList(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new()
                              .has_headers(false)
                              .from_reader(file);
    let mut vec = Vec::new();
    for result in rdr.records() {
        // let deserialized_row: [i32;record.len()] = record;
        // let deserialized_row: [i32;record.len()] = record.deserialize(None)?;
        // println!("{:?}", deserialized_row);
        let record = result?;
        for i in record.iter(){
            let num: i32 = i.parse()?;
            vec.push(num);
        } 
        break;
    }
    Ok(vec)
}


pub fn readCSVIntoStringList(filename: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new()
                              .flexible(true)
                              .has_headers(false)
                              .from_reader(file);
    let mut higherVec = Vec::new();
    
    for result in rdr.records() {
        let mut vec = Vec::new();
        let record = result?;
        for i in record.iter(){
            vec.push(i.to_owned());
        }
        higherVec.push(vec);
    }

    Ok(higherVec)
}
