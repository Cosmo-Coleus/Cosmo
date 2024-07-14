use super::InputMode;
use std::{
    fs::File,
    io::{BufReader, Read}, path::{Path, PathBuf},
};

/// Representation des donnees de l'editeur
pub struct Editor {
    pub input_mode: InputMode,
    pub scroll: (u16, u16),
    pub text_buffer: Vec<u8>,
    pub file_path: PathBuf
}

impl Editor {
    pub fn new() -> Self {
        let text = "".as_bytes();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(text);
        Self {
            input_mode: InputMode::Normal,
            scroll: (0, 0),
            text_buffer: buffer,
            file_path: PathBuf::new()
        }
    }

    pub fn open_file(self: &mut Editor, path: &String) {
        self.file_path = PathBuf::from(path);
        let file = File::open(&self.file_path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content).unwrap();
        self.text_buffer = Vec::from(content);
    }
}
