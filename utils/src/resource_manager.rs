use std::io;

pub fn check_resource(filename: &str) -> bool {
    let path = std::path::Path::new(filename);
    path.exists()
}

pub fn download_resource(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(url).expect("Failed to download resource");
    let mut file = std::fs::File::create(filename).expect("Failed to create resource file");
    io::copy(&mut response, &mut file).expect("Failed to copy resource file");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_resource() {
        // Test with a file that exists
        let existing_file = "Cargo.toml"; // Adjust the filename as needed
        assert!(check_resource(existing_file));

        // Test with a file that does not exist
        let non_existing_file = "non_existent_file.txt";
        assert!(!check_resource(non_existing_file));
    }

    #[test]
    fn test_download_resource() {
        let url = "https://www.rust-lang.org/logos/rust-logo-512x512.png"; // Example URL
        let filename = "test_resource.png";

        // Attempt to download the resource
        let result = download_resource(url, filename);
        assert!(result.is_ok());

        // Check if the file was created
        assert!(check_resource(filename));

        // Clean up the test file
        std::fs::remove_file(filename).expect("Failed to delete test file");
    }
}
