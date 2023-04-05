use std::path::Path;

// Utility function for getting the lines of a file.
pub fn read_file<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    match std::fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => panic!("Error reading file: {err}"),
    }
}
