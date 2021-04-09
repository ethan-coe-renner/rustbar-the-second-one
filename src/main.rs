use std::{thread, time};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt;

use chrono::Local;

fn main() {
    let delay = time::Duration::from_millis(6000);


    loop {
	let bar = Bar(
	    vec!(
		updates(),
		tasks(),
		news(),
		battery(),
	    )
	);
	println!("{}{}", bar, datetime());
	thread::sleep(delay);
    }
}

const WHITE: u8 = 0;
const GREY: u8 = 4;
const RED: u8 = 3;
const ORANGE: u8 = 2;

pub struct Bar(Vec<Widget>);

impl fmt::Display for Bar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let bar: String = self.0.iter().map(|x| x.to_string() + " | ").collect();

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
	write!(f, "+@fg={};{}: {}+@fg={};", self.color, self.name, self.data, WHITE)
    }
}

pub fn datetime() -> String {
    Local::now().format("%H:%M, %a %d %b %Y").to_string()
}

pub fn news() -> Widget {
    let unread = read_num_from_file("/home/ethan/.local/share/newsunread");
    Widget {
	name: "NEWS",
	data: unread.to_string(),
	color: if unread == 0 {GREY} else {WHITE},
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
	}
    }
}


pub fn tasks() -> Widget {
    let tasks = read_num_from_file("/home/ethan/.local/share/tasks");
    Widget {
	name: "TODO",
	data: tasks.to_string(),
	color: if tasks == 0 {GREY} else {WHITE},
    }
}

pub fn updates() -> Widget {
    let updates = read_num_from_file("/home/ethan/.local/share/updates");
    Widget {
	name: "UPD",
	data: updates.to_string(),
	color: if updates == 0 {GREY} else {WHITE},
    }
}

fn read_num_from_file(filepath: &'static str) -> u32 {
    let file = match File::open(&filepath) {
	Ok(file) => file,
	Err(_) => return 0
    };

    let mut buffer = BufReader::new(file);

    let mut line = String::new();
    let _ = buffer.read_line(&mut line);

    line.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap()
}
