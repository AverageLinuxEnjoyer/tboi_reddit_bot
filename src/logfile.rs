use std::{fs::OpenOptions, io::Read, time};

pub fn logfile(what: &str) {
    let now = time::SystemTime::now();
    let now = now.duration_since(time::UNIX_EPOCH).unwrap();

    let what = format!("[{}]:\t{}\n", now.as_secs(), what);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("logs")
        .unwrap();
}

pub fn read_logs(lines: usize) -> String {
    let mut file = OpenOptions::new().read(true).open("logs").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines().rev().take(lines).collect::<Vec<&str>>();
    lines.reverse();

    lines.join("\n")
}
