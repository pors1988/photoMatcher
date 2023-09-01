use std::{path::Path, io, collections::HashMap, fs::DirEntry};
use chrono::DateTime;
use std::time::Duration;
use humantime::{format_duration, format_rfc3339, format_rfc3339_seconds};

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

fn select_directory() -> Option<String> {
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

#[allow(dead_code)]
enum FileType {
    JPG,
    PNG,
}

#[allow(dead_code)]
impl FileType {
    fn as_str(&self) -> &str {
        match self {
            FileType::JPG => "jpg",
            FileType::PNG => "png",            
        }
    }
}
// struct FileData {
//     date_time: DateTime<chrono::Utc>,
//     path: DirEntry,
// }
#[allow(dead_code)]
struct FileCollector {
    all_files: Vec<DirEntry>,
    similar_files: Option<HashMap<chrono::NaiveTime, Vec<DirEntry>>>,
    file_type: Option<FileType>,
}

#[allow(dead_code)]
impl FileCollector {
    fn filter_by_type(&self) {
    }

    fn collect_similar_files(&self) {

    }

    fn print_collection(&self) {
        println!("Printing collection.");
        let beauty = "*-*------------------*-*";
        if !self.all_files.is_empty() {println!("{}", beauty); }
        for entry in &self.all_files {
            let file = std::fs::File::open(entry.path().clone())?;
            println!("{:?}, {:?}, {}",entry.file_name(), entry.file_type().unwrap(), entry.path().to_string_lossy());
            println!("{:?}", format_rfc3339_seconds(entry.metadata().unwrap().created().unwrap()).to_string());
            println!("{}", beauty);
        }

        println!("End printing collection.");
    }
}

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector_jpg = FileCollector{
        all_files: Vec::new(),
        similar_files: None,
        file_type: Some(FileType::JPG),
    };

    if let Some(directory) = select_directory() {
        println!("Selected directory: {}", directory);
        let path = Path::new(&directory);
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                collector_jpg.all_files.push(entry);
            }
        }
    } else {
        println!("No valid directory selected.");
    }
    // collector_jpg.print_collection();
    let path = collector_jpg.all_files.first().unwrap().path();
    let file = std::fs::File::open(path.clone())?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    println!("{}, {}", path.file_name().unwrap().to_str().unwrap(), exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY).unwrap().display_value());
    for f in exif.fields() {
        println!("{} {} {}",
                 f.tag, f.ifd_num, f.display_value().with_unit(&exif));
    }

    Ok(())
}