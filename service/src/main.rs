#![allow(dead_code)]
#![allow(unused_variables)]

use psutil::process::{self, Process};

fn sync() {
    // sync the saves at the start of the service
}

const GAMES: [&'static str; 3] = ["spotify", "code", "chrome"];

fn detect_game<'a>() -> Result<Vec<Process>, psutil::Error> {
    let mut detected: Vec<Process> = Vec::new();
    let mut names = Vec::new();

    for process_result in process::processes()? {
        if process_result.is_err() {
            continue;
        }

        let process = process_result.unwrap();

        if let Ok(name) = process.name() {
            // check if name is in the list and return the handle to its process

            if GAMES.contains(&name.as_str()) && !names.contains(&name) {
                detected.push(process);
                names.push(name);
            }
        }
    }

    Ok(detected)
}

fn main() {
    sync();

    #[allow(while_true)]
    while true {
        if let Ok(games) = detect_game() {
            let names = games
                .into_iter()
                .map(|game| format!("{}", game.name().unwrap()))
                .collect::<Vec<String>>();

            // spawn a thread for each game detected

            println!("{:?}", names);
            break;
        }
    }
}
