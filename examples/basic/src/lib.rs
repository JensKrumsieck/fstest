#[allow(dead_code)]
fn create_file() {
    //writes a file so test would pollute the workspace
    let file = std::fs::File::create("test.txt").unwrap();
    println!("File created: {:?}", file);
}

#[cfg(test)]
mod tests {
    use super::*;
    use fstest::fstest;
    use std::path::Path;

    #[fstest]
    fn test_create_file(dir: &Path) {
        create_file();

        assert!(dir.join("test.txt").exists(), "File should exist");
    }

    #[fstest(repo = true)]
    fn test_create_file_in_repo(dir: &Path) {
        create_file();

        assert!(dir.join("test.txt").exists(), "File should exist");
    }
}
