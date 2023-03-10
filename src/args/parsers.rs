pub fn repo_name_is_valid(arg: &str) -> Result<String, String> {
    if let Some((author, repository)) = arg.split_once('/') {
        if !(author.is_empty() || repository.is_empty()) {
            return Ok(String::from(arg));
        }
    }

    Err(String::from("invalid repository pattern"))
}
