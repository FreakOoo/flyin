#[derive(Debug)]
pub struct Drone
{
    pub    name: i32,
    pub    location: i32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZoneType
{
        Normal,
        Blocked,
        Restricted,
        Priority,
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

