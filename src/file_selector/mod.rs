pub use std::{path::Path, io};
fn read_line() -> io::Result<String> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    Ok(input)
}

fn pick_yes_no() -> bool {

    while let Ok(mut input) = read_line() {
        input = input.trim().to_lowercase();

        if input == "y" {
            return true;
        }
        else if input == "n" {
            return false;
        } else {
            println!("Please pick y or n");
        }
    }
    false
}

pub fn select_directory() -> Option<String> {
    let mut pick = true;
    let mut directory = String::new();

    while pick {
        let picked = xplr::runner::runner().and_then(|a| a.run());
        match picked {
            Ok(Some(out)) => {

                let dir = out.trim_end_matches('\n');
                if Path::new(dir).is_dir() {
                    directory = dir.to_string();
                    pick = false;
                } else {
                    println!("You picked file: {}", dir);
                    println!("Whould you try to pick directory or end program? type y - yes, n - end.");
                    pick = pick_yes_no();
                }
            }
            Ok(None) => {
                println!("No directory selected");
            }
            Err(err) => {
                if !err.to_string().is_empty() {
                    eprintln!("error: {}", err);
                }
                std::process::exit(1);
            }
        }
    }

    if !directory.is_empty() {
        Some(directory)
    } else {
        None
    }

}
