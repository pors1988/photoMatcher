use std::{path::Path, io, collections::HashMap, fs::DirEntry};
use chrono::{NaiveDateTime};

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
struct FileCollector{
    all_files: Vec<DirEntry>,
    similar_files: Option<HashMap<chrono::NaiveDateTime, Vec<std::path::PathBuf>>>,
    file_type: Option<FileType>,
    time_threshold: chrono::Duration,

}

#[allow(dead_code)]
impl FileCollector {
    fn new(time_threshold: chrono::Duration) -> Self {
        FileCollector{
            all_files: Vec::new(),
            similar_files: None,
            file_type: Some(FileType::JPG),
            time_threshold,
        }
    }

    fn filter_by_type(&self) {
    }

    fn collect_similar_files(&mut self) {
        if self.all_files.is_empty() {
            return;
        }
        let dir_entry = self.all_files.first().unwrap();
        let first_path = self.all_files.first().unwrap().path();
        let previous_created_date_time = self.get_entry_created_date_time(&dir_entry).unwrap(); 

        if let Some(similar_files) = &mut self.similar_files {
            similar_files
                .entry(previous_created_date_time)
                .or_insert_with(Vec::new)
                .push(first_path);
        }
        else {
            let mut new_map = HashMap::new();
            let mut new_paths = Vec::new();
            new_paths.push(first_path);
            new_map.insert(previous_created_date_time, new_paths);
            self.similar_files = Some(new_map);

        }


        let mut it = self.all_files.iter().skip(1);
        while let Some(entry) = it.next() {
           println!("{:?}", entry); 
        //    if let Some(recent_created_date_time) = self.get_entry_created_date_time(entry) {

        //    }
           break;
        }

    }

    fn get_entry_created_date_time(&self, dir_entry: &DirEntry) -> Option<NaiveDateTime> {
        let file = std::fs::File::open(dir_entry.path().clone()).unwrap();
        let mut bufreader = std::io::BufReader::new(file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader);
        let orginal_time  = exif.unwrap().get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY).unwrap().clone().display_value().to_string();
        let date_time = NaiveDateTime::parse_from_str(&orginal_time, "%Y-%m-%d %H:%M:%S").unwrap();
        Some(date_time)
    }

    fn print_collection(&self) {
        println!("Printing collection.");
        let beauty = "*-*------------------*-*";
        if !self.all_files.is_empty() {println!("{}", beauty); }
        for entry in &self.all_files {
            let date_time = self.get_entry_created_date_time(entry);

            println!("{:?}, {:?}",entry.file_name(), date_time.unwrap());
            println!("{}", beauty);
        }

        println!("End printing collection.");
    }
}

#[allow(dead_code)]
fn main() {
    let mut collector_jpg = FileCollector::new(chrono::Duration::seconds(120));

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
    collector_jpg.print_collection();

    collector_jpg.collect_similar_files();
}