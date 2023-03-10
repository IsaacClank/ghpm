use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::process::Command;
use std::str::FromStr;

/// Read and parse input from a reader
pub fn read_line_from<T: FromStr>(source: impl Read) -> Result<T, T::Err> {
    let mut reader = BufReader::new(source);
    let mut value = String::new();
    reader.read_line(&mut value).unwrap();
    value.trim().parse::<T>()
}

/// Extract an archive
pub fn extract_archive(path_to_archive: &str, destination: &str) {
    let extension = path_to_archive.split('.').last();

    match extension {
        Some("gz") | Some("xz") | Some("tar") => {
            fs::create_dir(destination).unwrap();

            Command::new("tar")
                .arg("-xf")
                .arg(path_to_archive)
                .arg("--directory")
                .arg(destination)
                .output()
                .unwrap();
        }

        Some("zip") | Some("7z") => {
            Command::new("unzip")
                .arg(path_to_archive)
                .arg("-d")
                .arg(destination)
                .output()
                .unwrap();
        }

        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn read_line_test_returns_err_if_cannot_parse_input() {
        let actual = read_line_from::<usize>(&mut "some input".as_bytes());
        assert!(actual.is_err());
    }

    #[test]
    pub fn read_line_test_returns_value_as_expected_type() {
        let expected = 156;
        let actual = read_line_from::<usize>(&mut expected.to_string().as_bytes());

        assert_eq!(Ok(expected), actual);
    }
}
