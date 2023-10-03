use std::path::PathBuf;
use std::collections::HashMap;
use chrono::NaiveDateTime;

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

#[allow(dead_code)]
pub struct FileCollector {
    all_files: Vec<std::path::PathBuf>,
    similar_files: Option<HashMap<chrono::NaiveDateTime, Vec<std::path::PathBuf>>>,
    file_type: Option<FileType>,
    time_threshold: chrono::Duration,

}

#[allow(dead_code)]
impl FileCollector {
    pub fn new(time_threshold: chrono::Duration) -> Self {
        FileCollector {
            all_files: Vec::new(),
            similar_files: None,
            file_type: Some(FileType::JPG),
            time_threshold,
        }
    }

    pub fn add_new_file(&mut self, path: PathBuf) {
        self.all_files.push(path);

    }
    pub fn collect_similar_files(&mut self) {
        if self.all_files.is_empty() {
            return;
        }

        let add_similar_file_entry = |similar_files: &mut Option<HashMap<chrono::NaiveDateTime,
            Vec<std::path::PathBuf>>>,
                                      created_date_time, first_path: Option<&PathBuf>| {
            similar_files
                .get_or_insert_with(|| {
                    let mut new_map = HashMap::new();
                    new_map.insert(created_date_time, Vec::new());
                    new_map
                })
                .entry(created_date_time)
                .or_insert_with(Vec::new)
                .push(first_path.unwrap().clone());
        };

        let mut prev_entry = None;

        let all_files_copy = self.all_files.clone();
        let mut i = 0;
        for entry in &all_files_copy {
            i = i + 1;
            // while let Some(entry) = it.next() {
            if prev_entry.is_none() {
                println!("{:?}", entry);
                let cdt = self.get_entry_created_date_time(entry).unwrap();
                let path = Some(entry);
                add_similar_file_entry(&mut self.similar_files, cdt, path);
            } else if let Some(recent_created_date_time) = self.get_entry_created_date_time(entry) {
                if let Some(previous_created_date_time) = self.get_entry_created_date_time(prev_entry.unwrap()) {
                    let time_diff = recent_created_date_time.timestamp() - previous_created_date_time.timestamp();
                    let key = if time_diff < self.time_threshold.num_seconds() { previous_created_date_time } else { recent_created_date_time };
                    add_similar_file_entry(&mut self.similar_files, key, Some(entry));
                }
            }
            prev_entry = Some(entry);
        }
        println!("What we got in similar files:");
        for (key, value) in self.similar_files.clone().unwrap().iter() {
            println!("{}: {:?}", key, value);
        }
    }

    fn get_entry_created_date_time(&self, dir_entry: &std::path::PathBuf) -> Option<NaiveDateTime> {
        let file = std::fs::File::open(dir_entry.clone()).unwrap();
        let mut bufreader = std::io::BufReader::new(file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader);
        let original_time = exif.unwrap().get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY).unwrap().clone().display_value().to_string();
        let date_time = NaiveDateTime::parse_from_str(&original_time, "%Y-%m-%d %H:%M:%S").unwrap();
        Some(date_time)
    }

    fn print_collection(&self) {
        println!("Printing collection.");
        let beauty = "*-*------------------*-*";
        if !self.all_files.is_empty() { println!("{}", beauty); }
        for entry in &self.all_files {
            let date_time = self.get_entry_created_date_time(entry);

            println!("{:?}, {:?}", entry.file_name(), date_time.unwrap());
            println!("{}", beauty);
        }

        println!("End printing collection.");
    }
}