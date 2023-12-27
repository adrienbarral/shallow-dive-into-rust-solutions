use std::{fs::File, io::Read, path::Path};

use thiserror::Error;
#[derive(Debug, Error)]
enum ParseError {
    #[error("Not enough element to parse, 2 are required {0} found")]
    NotEnoughElementError(usize),
    #[error("Not enough line in  file")]
    NotEnoughLineInFile,
}

fn parse_line(line: &str) -> anyhow::Result<(f32, f32)> {
    let res: Vec<&str> = line.split(";").collect();

    match (res.get(0), res.get(1)) {
        (Some(elem1), Some(elem2)) => {
            return Ok((elem1.parse::<f32>()?, elem2.parse::<f32>()?));
        }
        _ => return Err(ParseError::NotEnoughElementError(res.len()).into()),
    }
}

fn parse_from_file(file: &Path) -> anyhow::Result<(f32, f32)> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let first_line = contents
        .lines()
        .nth(0)
        .ok_or::<ParseError>(ParseError::NotEnoughLineInFile.into())?;

    parse_line(first_line)
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::{num::ParseFloatError, path::Path, io};

    use crate::*;

    #[test]
    fn ex6_parse_well_formated_line() {
        assert_eq!(parse_line("12.5;-5.2").unwrap(), (12.5, -5.2));
    }

    #[test]
    fn ex6_parse_non_float_number() {
        let parse_error = parse_line("12.5xezf;-5.2").err().unwrap();
        assert_eq!(parse_error.is::<ParseFloatError>(), true);
    }

    #[test]
    fn ex6_parse_line_with_1_elemnt() {
        let parse_error = parse_line("-5.2").err().unwrap();
        assert_eq!(parse_error.is::<ParseError>(), true);
    }

    #[test]
    fn ex6_open_and_parse_from_correct_file() {
        let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("correct_file.mir");

        // Un comment and create method making this test passing :
        assert_eq!(parse_from_file(&file_path).unwrap(), (12.5, -2.45));
    }

    #[test]
    fn ex6_open_and_parse_from_incorrect_file() {
        let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("dontexists.mir");

        let parse_error = parse_from_file(&file_path).err().unwrap();
        assert_eq!(parse_error.is::<io::Error>(), true);
    }
}
