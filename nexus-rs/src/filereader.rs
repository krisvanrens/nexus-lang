use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

/// Iterable wrapper around a buffered file reader.
pub struct FileReader {
    reader: BufReader<File>,
}

impl FileReader {
    pub fn try_new<P>(filename: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        match File::open(&filename) {
            Ok(file) => Ok(FileReader {
                reader: BufReader::new(file),
            }),
            Err(error) => Err(error),
        }
    }
}

impl IntoIterator for FileReader {
    type Item = String;
    type IntoIter = FileReaderIterator;

    fn into_iter(self) -> Self::IntoIter {
        FileReaderIterator {
            lines: self.reader.lines(),
        }
    }
}

pub struct FileReaderIterator {
    lines: Lines<BufReader<File>>,
}

impl Iterator for FileReaderIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.lines.next() {
            let line = result.expect("failed to read line");
            Some(line)
        } else {
            None
        }
    }
}
