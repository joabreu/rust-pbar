extern crate colorful;

use std::{thread, time};
use std::io::{stdin, Read};
use colorful::Colorful;
use colorful::HSL;

fn main() {
    let mut s: &str = "\\";
    print!("{}[2J", 27 as char);
    println!("{}\n", "Most Loved, Dreaded, and Wanted Languages".red());
    let values = vec![78.9, 75.1, 68.0, 67.0, 65.6, 65.1, 61.9, 60.4];
    let languages = vec!["Rust", "Kotlin", "Python", "TypeScript", "Go", "Swift", "JavaScript", "C#"];
    let c = languages.iter().max_by_key(|x| x.len()).unwrap();
    let mut finished = 0;
    let mut i = 0;

    while finished < languages.len() {
        let mut tmp = vec![];
        finished = 0;

        for (j, value) in values.iter().enumerate() {
            let mut new_value: f64 = (i as f64 / 10.00) + (*value / 10.00);
            if new_value >= *value {
                new_value = *value;
                finished += 1;
            }
            let h = (new_value as f32 * 15.0 % 360.0) / 360.0;
            let length = new_value as usize;
            let real_text = format!(
                "{:<width$} | {} {:.2}%",
                languages.get(j).unwrap(),
                s.repeat(length).gradient(HSL::new(h, 1.0, 0.5)),
                new_value,
                width = c.len()
            );

            tmp.push(real_text);
            s = if i % 2 != 0 { "\\" } else { "/" };
            i += 1;
        }

        println!("{}\x1B[{}F\x1B[G\x1B[2K", tmp.join("\n"), languages.len());
        thread::sleep(time::Duration::from_millis(50)); 
    }

    print!("{}[2J", 27 as char);
    let mut buffer = [0];
    _ = stdin().read_exact(&mut buffer);
}