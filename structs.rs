#[derive(Debug)]
pub struct Drone
{
    pub    name: i32,
    pub    location: i32,
}
#[derive(Debug)]
pub enum Attribute
{
        Normal,
        Blocked,
        Priority,
        Contested,
        Start,
        End,
}
#[derive(Debug)]
pub struct Zone 
{
    pub    name: i32,
    pub    n_connections: i32,
    pub    capacity: i32,
    pub    attributes: Attribute,

