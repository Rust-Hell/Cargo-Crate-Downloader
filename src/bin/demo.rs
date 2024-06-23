use std::fs::{create_dir, remove_dir_all};
use std::path::Path;

use downloader::Downloader;

use crate_downloader::{deal_with_crate, fetch_crate};

const CRATESDIR: &str = "./demo";


fn main() {
    let name = "rand";
    let version_num = "0.8.5";

    // Main Process
    println!("Processing crate {name}-v{version_num}");
    remove_dir_all(CRATESDIR).unwrap_or_default(); // Delete tmp crates file directory
    create_dir(Path::new(CRATESDIR)).unwrap_or_default(); // Crates file directory
    let mut downloader = Downloader::builder()
        .download_folder(Path::new(CRATESDIR))
        .parallel_requests(1)
        .build()
        .expect("Fatal Error, build downloader fails!");

    if let Err(e) =
        fetch_crate( &mut downloader, CRATESDIR, &name, &version_num)
    {
        println!("Fetch fails: {}",  e);
    } else if let Err(e) = deal_with_crate(CRATESDIR, &name, &version_num) {
        println!("Unzip fails: {}", e);
    } else {
        println!("Success. Check the directory ./{}", CRATESDIR);
    }

}
