
use std::sync::mpsc::Receiver;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn progress_bar(rx: Receiver<Duration>) {
    let total_duration = 10; // 15 seconds to complete
    let bar_length = 50; // Length of the progress bar
    let sleep_duration = Duration::from_millis((total_duration * 1000) / bar_length as u64);

    //Clear terminal
    print!("{}[2J", 27 as char);

    println!("============== Raytracing tip of the day ==============");
    //read content of answer.txt and print it
    let contents = std::fs::read_to_string("answer.txt").expect("Something went wrong reading the file");
    println!("{}", contents);
    println!("========================================================");
    println!("Rendering started. Please wait...");

    let mut direction = 1isize; // 1 for moving right, -1 for moving left
    let mut position = 0;

    loop {
        if rx.try_recv().is_ok() || position == bar_length {
            println!("\r[{: <50}] Rendering completed.", "=".repeat(position));
            break;
        }

        let progress_bar = format!(
            "\r[{: <width$}{}{: >remaining_width$}]",
            " ".repeat(position),
            "╭∩╮( •̀_•́ )╭∩╮",
            " ",
            width = position,
            remaining_width = bar_length - position - 1
        );

        print!("{}", progress_bar);
        io::stdout().flush().unwrap();

        thread::sleep(sleep_duration);
        
        position = (position as isize + direction) as usize;
        if position == bar_length - 1 || position == 0 {
            direction *= -1; // Change direction when reaching the ends
        }
    }

    println!();
}

