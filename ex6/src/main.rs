#[derive(Debug, PartialEq)]
enum ParseError {
    ParseError,
    NotEnoughElementError,
    CantOpenFile,
}

// Exercice : On laisse écrire toute cette fonction avec sa signature
// (on laisse l'enum ci-dessus).
fn parse_line(line: &str) -> Result<(f32, f32), ParseError> {
    let res: Vec<&str> = line.split(";").collect();

    match (res.get(0), res.get(1)) {
        (Some(elem1), Some(elem2)) => {
            if let Ok(val1) = elem1.parse::<f32>() {
                if let Ok(val2) = elem2.parse::<f32>() {
                    return Ok((val1, val2));
                }
            }
            return Err(ParseError::ParseError);
        }
        _ => return Err(ParseError::NotEnoughElementError),
    }
}

fn parse_from_file(file_path: &std::path::Path) -> Result<(f32, f32), ParseError> {
    let content = std::fs::read_to_string(file_path);
    match content {
        Ok(content) => {
            let lines: Vec<&str> = content.split("\n").collect();
            if let Some(line) = lines.get(0) {
                return parse_line(line);
            }
            return Err(ParseError::NotEnoughElementError);
        }
        Err(_) => return Err(ParseError::CantOpenFile),
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::*;

    #[test]
    fn ex6_parse_well_formated_line() {
        assert_eq!(parse_line("12.5;-5.2"), Ok((12.5, -5.2)));
    }

    #[test]
    fn ex6_parse_non_float_number() {
        assert_eq!(parse_line("12.5xezf;-5.2"), Err(ParseError::ParseError));
    }

    #[test]
    fn ex6_parse_line_with_1_elemnt() {
        assert_eq!(parse_line("-5.2"), Err(ParseError::NotEnoughElementError));
    }

    #[test]
    fn ex6_parse_from_correct_file() {
        let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("correct_file.mir");

        // Un comment and create method making this test passing :
        assert_eq!(parse_from_file(&file_path), Ok((12.5, -2.45)));
    }

    #[test]
    fn ex6_parse_from_incorrect_file() {
        let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_resources")
            .join("dontexists.mir");

        // Un comment and create method making this test passing :
        assert_eq!(parse_from_file(&file_path), Err(ParseError::CantOpenFile));
    }
}
