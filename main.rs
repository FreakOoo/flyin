mod structs;
mod parsing;

// currently this is just for test printing 
// I slopped the printout text


use structs::{Drone, Zone, Attribute};

fn main()
{
    let drone = Drone {
        name: 1,
        location: 1,
    };
    let zone = Zone {
        name: 1,
        n_connections: 1,
        capacity: 1,
        attributes: Attribute::Start,
    };
    println!(
        "Drone {} is currently located in zone {}.",
        drone.name, drone.location
    );

    println!(
        "Zone {} has {} connections, a capacity of {}, and an attribute of {:?}.",
        zone.name, zone.n_connections, zone.capacity, zone.attributes
    );

    // Print every field of each struct automatically:
    println!("Full drone details: {:?}", drone);
    println!("Full zone details: {:?}", zone);
}
