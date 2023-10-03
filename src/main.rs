mod file_collector;
use file_collector::FileCollector;
mod file_selector;
use file_selector::select_directory;
use file_selector::Path;

#[allow(dead_code)]
fn main() {
    let mut collector_jpg = FileCollector::new(chrono::Duration::seconds(120));

    if let Some(directory) = select_directory() {
        println!("Selected directory: {}", directory);
        let path = Path::new(&directory);
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                collector_jpg.add_new_file(entry.path());
            }
        }
    } else {
        println!("No valid directory selected.");
    }
    // collector_jpg.print_collection();

    collector_jpg.collect_similar_files();
}