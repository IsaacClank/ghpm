use std::{fmt::Debug, fs, io, process::Command, str::FromStr};

/// Read a line and return the parsed value of type T
pub fn read_line<T: FromStr>() -> Result<T, T::Err>
where
  <T as FromStr>::Err: Debug,
{
  let mut value = String::new();

  io::stdin().read_line(&mut value).unwrap();
  let value = value.trim().parse::<T>()?;

  Ok(value)
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
