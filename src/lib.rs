use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

// constants as defined in spectrwm config
const WHITE: u8 = 0;
const GREY: u8 = 4;
const RED: u8 = 3;
const ORANGE: u8 = 2;

pub struct Bar(pub Vec<Widget>);

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bar: String = self
            .0
            .iter()
            .map(|x| x.to_string() + " +@fg=4;|+@fg=0; ")
            .collect();

        write!(f, "{}", bar)
    }
}

pub struct Widget {
    name: &'static str,
    data: String,
    color: u8,
}

impl fmt::Display for Widget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "+@fg={};{}: {}+@fg={};",
            self.color, self.name, self.data, WHITE
        )
    }
}

pub fn news() -> Widget {
    let unread = read_num_from_file("/home/ethan/.local/share/newsunread");
    Widget {
        name: "NEWS",
        data: unread.to_string(),
        color: if unread == 0 { GREY } else { WHITE },
    }
}

pub fn battery() -> Widget {
    let bat0percent = read_num_from_file("/sys/class/power_supply/BAT0/capacity");
    let bat1percent = read_num_from_file("/sys/class/power_supply/BAT1/capacity");

    Widget {
        name: "BAT",
        data: (bat0percent + bat1percent).to_string(),
        color: match bat0percent + bat1percent {
            x if x > 100 => WHITE,
            x if x > 50 => ORANGE,
            _ => RED,
        },
    }
}

pub fn tasks() -> Widget {
    let task = String::from_utf8(
        Command::new("task")
            .arg("rc.verbose:")
            .arg("limit:1")
            .arg("statbar")
            .output()
            .expect("failed to get task")
            .stdout,
    )
	.unwrap();
    Widget {
        name: "TODO",
        color: if task.is_empty() { GREY } else { WHITE },
        data: if task.is_empty() {
            "none".to_string()
        } else {
            task.trim().to_string()
        },
    }
}

pub fn updates() -> Widget {
    let updates = read_num_from_file("/home/ethan/.local/share/updates");
    Widget {
        name: "UPD",
        data: updates.to_string(),
        color: if updates == 0 { GREY } else { WHITE },
    }
}

pub fn volume() -> Widget {
    let volume: u32 = String::from_utf8(
        Command::new("pamixer")
            .arg("--get-volume")
            .output()
            .expect("failed to get volume")
            .stdout,
    )
	.unwrap()
	.chars()
	.filter(|c| c.is_digit(10))
	.collect::<String>()
	.parse::<u32>()
	.unwrap();

    let muted: bool = String::from_utf8(
        Command::new("pamixer")
            .arg("--get-mute")
            .output()
            .expect("failed to get mute status")
            .stdout,
    )
	.unwrap()
        == "true\n";

    Widget {
        name: "VOL",
        data: volume.to_string(),
        color: if muted { GREY } else { WHITE },
    }
}

pub fn music() -> Widget {
    let music_info = String::from_utf8_lossy(
        &Command::new("cmus-remote")
            .arg("-C")
            .arg("format_print '%a - %t'")
            .output()
            .expect("failed to get music_info")
            .stdout,
    )
	.replace("\n", "");

    Widget {
        name: "MUS",
        color: if music_info.is_empty() { GREY } else { WHITE },
        data: if music_info.is_empty() {
            "none".to_string()
        } else {
            music_info
        },
    }
}


pub fn network() -> Widget {
    let net_name = String::from_utf8_lossy(
        &Command::new("nmcli")
            .arg("-t")
            .arg("-f")
            .arg("NAME")
            .arg("c")
            .arg("show")
            .arg("--active")
            .output()
            .expect("failed to get network information")
            .stdout,
    )
	.replace("\n", "");

    Widget {
        name: "NET",
        color: if net_name.is_empty() { GREY } else { WHITE },
        data: if net_name.is_empty() {
            "none".to_string()
        } else {
            net_name
        },
    }
}

fn read_num_from_file(filepath: &'static str) -> u32 {
    let file = match File::open(&filepath) {
        Ok(file) => file,
        Err(_) => return 0,
    };

    let mut buffer = BufReader::new(file);

    let mut line = String::new();
    let _ = buffer.read_line(&mut line);

    line.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}
