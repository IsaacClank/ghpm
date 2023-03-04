use std::{fs, io, process::Command, str::FromStr};

/// Read and parse input from a reader
pub fn read_line_from<T: FromStr>(reader: &mut impl io::Read) -> Result<T, T::Err> {
    let mut value = String::new();
    reader.read_to_string(&mut value).unwrap_or_default();
    value.trim().parse::<T>()
}

/// Extract an archive
pub fn extract_archive(path: &str, destination: &str) {
    let extension = path.split('.').last();

    match extension {
        Some("gz") | Some("xz") | Some("tar") => {
            fs::create_dir(destination).unwrap();

            Command::new("tar")
                .arg("-xf")
                .arg(path)
                .arg("--directory")
                .arg(destination)
                .output()
                .unwrap();
        }

        Some("zip") | Some("7z") => {
            Command::new("unzip")
                .arg(path)
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
