use std::fs::File;
use std::io::{Write, Error};

pub fn write_json(json: String) -> Result<(), Error> {
    let path = "./tmp/result.txt";

    let mut output = File::create(path)?;
    write!(output, "{}", json)?;

    Ok(())
}
