use std::error::Error;

use std::fs;
use std::io;
use std::process;
use std::fs::File;

// Todo: find better way to deserilize
pub fn readCSVIntoIntList(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new()
                              .has_headers(false)
                              .from_reader(file);
    let mut vec = Vec::new();
    for result in rdr.records() {

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
