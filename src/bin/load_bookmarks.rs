use serde::Deserialize;
use serde_json;
// use diesel_demo::{*, schema::bookmarks};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::Error;

#[derive(Deserialize)]
struct BookmarkJson {
    url: String,
    title: Option<String>,
    // tags: 
}

// get all files in a given directory
fn get_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, Error> {

  let mut files = Vec::new();

  for entry in fs::read_dir(path)? {
    let entry = entry?;
    if entry.metadata()?.is_file() {
      files.push(entry.path());
    }
  }

  Ok(files)
}

fn main() {
  // let connection = &mut establish_connection();

  // get a list of file in data directory
  let dir_path = "./data";
  match get_files(dir_path) {
    Ok(files) => {
      for file in files {

        println!("{}", file.display());

        // read the files contents
        let file = File::open(file).expect("Failed to open file");
        let bookmarks: Vec<BookmarkJson> = serde_json::from_reader(file).expect("Failed to parse JSON");
    
        for bookmark in bookmarks.iter() {
          println!("  {}: {:?}", bookmark.url, bookmark.title);
        }
    
      }      
    }
    Err(e) => {
      eprintln!("Error reading directory: {}", e);
    }
  }

  // for file_path in files.iter() {
  //   let file = File::open(file_path).expect("Failed to open file");
  //   let bookmarks: Vec<BookmarkJson> = serde_json::from_reader(file).expect("Failed to parse JSON");

  //   println!("{}:", file_path);
  //   for bookmark in bookmarks.iter() {
  //     println!("  {}: {}", bookmark.title, bookmark.url);
  //   }
  // }
  // let website = create_website(connection, &host);

}
