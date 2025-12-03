pub mod p1;
pub mod p2;

#[derive(Debug)]
pub struct Battery {
    cells: Vec<u8>,
}

impl Battery {
    pub fn parse(input: &str) -> Battery {
        Battery {
            cells: input.chars().map(|c| c as u8 - 48).collect(),
        }
    }
}

pub fn parse(input: &str) -> Vec<Battery> {
    input.lines().map(Battery::parse).collect()
}
