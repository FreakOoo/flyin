#[derive(Debug)]
pub struct Drone
{
    pub    name: usize,
    pub    location: usize,
}
impl Drone
{
    pub fn new(id:usize, location:usize)->Drone 
    {
        Drone { name: id, location}
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoneType
{
        Normal,
        Blocked,
        Restricted,
        Priority,
}
//from and towards are just names, connections are 
//bidirectional, I may rename them to a and b
#[derive(Debug)]
pub struct Connection
{
    pub from:usize,
    pub towards:usize,
    pub max_link_capacity:usize,
}

#[derive(Debug)]
pub struct Zone 
{
    pub    name: String,
    pub    x: i32,
    pub    y: i32,
    pub    zone_type: ZoneType,
    pub    max_drones: usize,
    pub    color: Option<String>,
    pub    neighbors: Vec<usize>,
}

