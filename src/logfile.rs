use ::time::{format_description, OffsetDateTime};
use anyhow::Result;
use shuttle_service::tracing_subscriber::fmt::format;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    time,
};

use tracing::info;

pub fn logfile(what: &str) -> Result<()> {
    let now = OffsetDateTime::from_unix_timestamp(
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)?
            .as_secs() as i64,
    )?;

    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;

    let now = now.format(&format)?;

    let what = format!("[{}]:\t{}\n", now, what);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("logs.txt")?;

    file.write_all(what.as_bytes())?;

    Ok(())
}

pub fn read_logs(lines: usize) -> String {
    logfile("Requested logs.");
    let mut file = match OpenOptions::new().read(true).open("logs.txt") {
        Ok(file) => file,
        Err(err) => return err.to_string(),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines().rev().take(lines).collect::<Vec<&str>>();
    lines.reverse();

    lines.join("\n")
    // "lmao".to_string()
}
