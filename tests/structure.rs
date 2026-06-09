use std::{fs, path::Path};

const PREFERRED_LINES: usize = 100;
const MAX_LINES: usize = 150;

#[test]
fn production_files_are_small_and_have_no_tests() {
    let mut files = Vec::new();
    collect_rs_files(Path::new("src"), &mut files);
    for path in files {
        let content = fs::read_to_string(&path).unwrap();
        let lines = content.lines().count();
        assert!(
            lines <= MAX_LINES,
            "{} has {lines} lines; maximum is {MAX_LINES}",
            path.display()
        );
        if lines > PREFERRED_LINES {
            eprintln!(
                "warning: {} has {lines} lines; prefer at most {PREFERRED_LINES}",
                path.display()
            );
        }
        assert!(
            !content.contains("#[test]") && !content.contains("#[cfg(test)]"),
            "{} contains tests; move them under tests/",
            path.display()
        );
    }
}

fn collect_rs_files(directory: &Path, files: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_rs_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}
