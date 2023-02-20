use anyhow::{Result, anyhow};
pub mod auth;
pub mod processes;

const ILLEGAL_STRINGS: [&'static str; 14] = ["drop", "table", "insert", "modify", "where", ";", ",", r"\0", r"\n", r"\r", r"\", r"'", "\"", r"\z"];

fn sanitize_string(str: String) -> Result<()> {
    let lower_case = str.to_lowercase();
    for illegal_string in ILLEGAL_STRINGS {
        if lower_case.contains(illegal_string) {return Err(anyhow!("Illegal string"))}
    }
    Ok(())
}

