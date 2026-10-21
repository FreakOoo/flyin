//left off here 
//still need to complete the 
//turning these key value pairs 
//into usable keys and values in other files.
//I'm using a hashmap to make the values easy to retrieve
//from the keys
//
//was still building this and still need help for it 
//

use std::collections::HashMap;
use std::fs;

pub fn read_config()
{
    let info = fs::read_to_string("config.txt")
        .expect("Couldn't read config properly");

    for line in info.lines()
    {
        //this is while I'm still testing if reading the config even works
        //let line = line.trim();
        println!("Line: {}", line);
    }
}


