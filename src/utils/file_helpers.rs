use std::io::Write;
use std::path::PathBuf;

pub fn with_tempfile(content: &str, closure: fn(String) -> ()) {
    let mut file = tempfile::Builder::new()
        .tempfile()
        .expect("Cannot make temporary file.");

    file.write_all(content.as_bytes())
        .expect("Cannot write content in created temp file.");

    let path = file
        .path()
        .to_str()
        .expect("Cannot get path of temporary file")
        .to_string();

    closure(path);
}

pub fn with_tempdir(closure: impl Fn(String) -> ()) {
    let dir = tempfile::Builder::new()
        .tempdir()
        .expect("Cannot make temporary directory.");

    let path = dir
        .path()
        .to_str()
        .expect("Cannot get path of temporary directory.")
        .to_string();

    closure(path);
}

pub fn with_named_file(
    parent: &str,
    name: &str,
    content: &str,
    closure: impl Fn() -> (),
) {
    let mut pathbuf = PathBuf::from(parent);

    pathbuf.push(name);

    let filepath = pathbuf.to_str().expect("Cannot make filepath").to_string();

    let mut file =
        std::fs::File::create(&filepath).expect("Cannot create file");

    file.write_all(content.as_bytes())
        .expect("Cannot write content");

    file.flush().expect("Failure during flushing.");

    closure();

    std::fs::remove_file(&filepath).expect("Failed removing temporary file.");
}

pub fn with_named_dir(
    parent: &str,
    name: &str,
    closure: impl Fn(String) -> (),
) {
    let mut pathbuf = PathBuf::from(parent);

    pathbuf.push(name);

    let filepath = pathbuf.to_str().expect("Cannot make filepath,").to_string();

    std::fs::DirBuilder::new()
        .create(&filepath)
        .expect("Cannot create temporary directory.");

    closure(filepath.clone());

    std::fs::remove_dir(&filepath).expect("Failed removing temporary file.");
}

#[cfg(test)]
pub fn with_working_directory(
    working_directory: &str,
    closure: impl Fn() -> (),
) {
    let previous_current_dir =
        std::env::current_dir().expect("Cannot get working directory.");

    std::env::set_current_dir(working_directory)
        .expect("Cannot set working directory.");

    closure();

    std::env::set_current_dir(previous_current_dir)
        .expect("Cannot set working directory.");
}
