use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

/// Iterable wrapper around a buffered file reader.
///
/// The main difference to using the buffered file reader directly is that each line reading iteration results in a
///  `String` value directly, rather than a `Option<Result<String>>` (which is the case for iterating over `BufReader`).
/// A failing line read will result in a panic (don't use in production!).
///
/// # Example
///
/// ```no_run
/// use nexus_rs::filereader::FileReader;
///
/// let file = FileReader::try_new("example.txt").unwrap();
///
/// for line in file {
///   println!("{line}");
/// }
/// ```
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

/// Iterator for [FileReader].
pub struct FileReaderIterator {
    lines: Lines<BufReader<File>>,
}

impl Iterator for FileReaderIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.lines.next() {
            let line = result.unwrap_or_else(|e| {
                panic!("failed to read line ({e})");
            });
            Some(line)
        } else {
            None
        }
    }
}
