use std::fs;
use std::collections::HashMap;



pub fn read_config()->HashMap<String, String>

{
    let mut config = HashMap::new();

    let info = fs::read_to_string("config.txt")
        .expect("Couldn't read config properly");

    for line in info.lines()
    {

        let line = line.trim();
        if line.is_empty() || line.starts_with('#')
        {
            continue;
        }            
        //Some is for if I may or may not get a result
        //if no result skip, if result take
        //unlike expect that throws an error if empty. (I think)
        if let Some((key,value)) = line.split_once('=')
        {
            config.insert(
                key.trim().to_string(),
                value.trim().to_string(),
            );
        }

    }

    config
}
