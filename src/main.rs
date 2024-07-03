use flate2::{Compression, GzBuilder};
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use std::env;

#[derive(Debug)]
struct FileReader {
    path: PathBuf,
}

impl FileReader {
    fn new() -> Self {
        let arg = env::args().nth(1).expect("Couldnt Find path");
        let path = Path::new(&arg).to_owned();
        if path.is_dir() {
            panic!("Should be file find dir");
        };
        Self { path }
    }
}

struct FileHandler;
impl FileHandler {
    fn compress(file_reader: &FileReader) -> std::io::Result<()> {
        let mut content: Vec<u8> = Vec::new();
        let _ = File::open(&file_reader.path)?.read_to_end(&mut content)?;
        let file_name = file_reader.path.file_name().unwrap().to_str().unwrap();
        let path_for_compressed_file = String::from(file_reader.path.to_str().unwrap()) + ".gz";

        let mut gz = GzBuilder::new().filename(file_name).write(
            File::create(path_for_compressed_file)?,
            Compression::default(),
        );

        gz.write_all(&content)?;
        gz.finish()?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let file_reader = FileReader::new();
    FileHandler::compress(&file_reader)?;

    Ok(())
}
