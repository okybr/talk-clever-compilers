use std::error::Error;
use std::io;
use std::result::Result;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let size: usize = input.trim().parse()?;

    let mut array: Vec<i32> = Vec::with_capacity(size);
    let another_reference: &mut Vec<i32> = &mut array;

    another_reference.push(1);
    array.push(2);

    Ok(())
}
