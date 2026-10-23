mod structs;
mod parsing;

// currently this is just for test printing
// I slopped the printout text

use structs::{Drone, Zone, ZoneType};
use parsing::read_config;

fn main()
{
    let drone = Drone {
        name: 1,
        location: 1,
    };
    let zone = Zone {
        name: String::from("hub"),
        x: 0,
        y: 0,
        zone_type: ZoneType::Normal,
        max_drones: 1,
        color: None,
        neighbors: vec![1, 2],
    };
    println!(
        "Drone {} is currently located in zone {}.",
        drone.name, drone.location
    );

    println!(
        "Zone {} has {} connections, a capacity of {}, and a type of {:?}.",
        zone.name, zone.neighbors.len(), zone.max_drones, zone.zone_type
    );

    // Print every field of each struct automatically:
    println!("Full drone details: {:?}", drone);
    println!("Full zone details: {:?}", zone);
    println!("Now doing the config printout");
    let config = read_config();
    println!("\n now printing from key value pairs in config\n\n");
    for(key,value) in &config
    {
        println!("{key} = {value}");
    }
}
