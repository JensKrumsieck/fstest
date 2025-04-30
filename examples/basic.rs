fn create_file() {
    //writes a file so test would pollute the workspace
    let file = std::fs::File::create("test.txt").unwrap();
    println!("File created: {:?}", file);
}

fn main() {
    create_file();
}

#[cfg(test)]
mod tests {
    use super::*;
    use fstest::fstest;
    use std::path::Path;

    #[fstest]
    fn test_create_file() {
        create_file();

        assert!(Path::new("test.txt").exists(), "File should exist");
    }

    #[fstest(repo = true)]
    fn test_create_file_in_repo() {
        create_file();

        assert!(Path::new("test.txt").exists(), "File should exist");
    }

    #[fstest(files = ["examples/example.txt"] )]
    fn test_create_file_with_files() {
        create_file();

        assert!(Path::new("test.txt").exists(), "File should exist");
        assert!(Path::new("example.txt").exists(), "File should exist");
    }
}
