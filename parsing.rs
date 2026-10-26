use std::fs;
use crate::structs::Zone;



pub struct Config
{
    pub nb_drones: usize,
    pub zones: Vec<Zone>,
    pub start: usize,
    pub end: usize,
}

// result(T,E) is common to denote expected type,
// and the error type in case it fails.

fn load_file(path: &str) -> Result<String, String>
{
    fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {}", path, e))
}

fn is_skippable(line: &str) -> bool
{
    line.is_empty() || line.starts_with('#')
}

fn split_line(line: &str, line_no: usize) -> Result<(&str, &str), String>
{
    match line.split_once(':')
    {
        Some((prefix, rest)) => Ok((prefix.trim(), rest.trim())),
        None => Err(format!(
            "line {}: missing ':' in '{}'", line_no, line)),
    }
}


fn parse_nb_drones(rest: &str, line_no: usize) -> Result<usize, String>
{
    let n = rest.parse::<usize>()
        .map_err(|_| format!(
            "line {}: invalid drone count '{}'", line_no, rest))?;
    if n == 0
    {
        return Err(format!(
            "line {}: drone count must be positive", line_no));
    }
    Ok(n)
}

fn split_metad(remainder: &str, line_n: usize)->Result<(&str, Vec<(&str, &str)>),String>
{
    let open = match remainder.find('[')
    {
        Some(i) => i,
        None => return Ok((remainder, Vec::new())),
    };
    if !remainder.ends_with(']')
    {
        return Err(format!("line {}: missing ']'", line_n));
    }
    let mut pairs = Vec::new();
    for token in remainder[open + 1..remainder.len() - 1].split_whitespace()
    {
        match token.split_once('=')
        {
            Some((key, value)) => pairs.push((key, value)),
            None => return Err(format!(
                "line {}: bad metadata '{}'", line_n, token)),
        }
    }
    Ok((remainder[..open].trim(), pairs))
}

pub fn read_config()->Result<Config,String>
{
    let info = load_file("config.txt")?;
    let mut nb_drones: usize = 0;
    let mut first_line = true;

    for (i, line) in info.lines().enumerate()
    {
        let line = line.trim();
        if is_skippable(line)
        {
            continue;
        }
        let (prefix, rest) = split_line(line, i + 1)?;
        if first_line
        {
            if prefix != "nb_drones"
            {
                return Err(format!(
                    "line {}: first line must be nb_drones", i + 1));
            }
            nb_drones = parse_nb_drones(rest, i + 1)?;
            first_line = false;
        }
    }
    println!("nb_drones = {}", nb_drones);
    Err(String::from("parser not finished yet"))
}

