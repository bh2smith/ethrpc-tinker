use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use solabi::Address;

fn read_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        data.push(line?);
    }

    Ok(data)
}

pub fn addresses_from_file(file_path: &str) -> Result<Vec<Address>, Box<dyn Error>> {
    Ok(read_file(file_path)?
        .iter()
        .map(|s| Address::from_str(s).unwrap())
        .collect())
}
