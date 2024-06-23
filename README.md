# Cargo-Crate-Downloader


### Download the entire crates.io

For test and direct usage, you can use another binary that will download a single crate into the target directory:
```Shell
// For example: cargo run --bin download_one -- rusqlite 0.29.0
cargo run --bin download_one -- <name> <version_num>
```

We provide a demo for you to learn how to download crates.

```Rust
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

```


### Download the entire crates.io

The crate version info are based on DB of our [Cargo-Ecosystem-Monitor](https://github.com/Cargo-Ecosystem-Monitor). Follow the guide in the root directory.

In this project, we will download and decompress all crates source code from crates.io (Cargo Ecosystem). The process can be broken and re-started, as we support break-point execution. The data will be stored in `CRATESDIR`(./on_process by default, see code to change it). 

Directly running the project requires preliminary DB support. Make sure you are prepared to do so.
```Shell
cargo run
```