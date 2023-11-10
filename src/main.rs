use std::time::SystemTime;

fn main() {
    let mut latest_time: SystemTime = SystemTime::now();

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: wasm-pack-watch <watch_dir> <build_dir>");
        std::process::exit(1);
    }

    let watch_dir = &args[1];
    let build_dir = &args[2];

    println!("Waiting for a file to change.");

    loop {
        // sleep for three seconds
        std::thread::sleep(std::time::Duration::from_secs(3));

        let found_newer_file = find_newer_file(&watch_dir, &mut latest_time);

        // if a file was newer than the latest time, run the wasm-pack build command
        if !found_newer_file {
            continue;
        }

        let mut builder = match std::process::Command::new("wasm-pack")
            .arg("build")
            .arg(&watch_dir)
            .arg("--target")
            .arg("web")
            .arg("--out-dir")
            .arg(&build_dir)
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                println!("Failed to spawn child process: {}", e);
                continue;
            }
        };

        let output = match builder.wait() {
            Ok(output) => output,
            Err(e) => {
                println!("Failed to wait on child process: {}", e);
                continue;
            }
        };

        if !output.success() {
            println!("Failed to build: {}", output);
        }
    }
}

fn find_newer_file(watch_dir: &str, latest_time: &mut SystemTime) -> bool {
    // search for files in the current directory
    let files = std::fs::read_dir(watch_dir).unwrap();

    // iterate over the files and alert if one is newer

    for file in files {
        let file = file.unwrap();

        let file_type = file.file_type().unwrap();
        if file_type.is_dir() {
            match find_newer_file(file.path().to_str().unwrap(), latest_time) {
                true => return true,
                false => continue,
            };
        } else if file_type.is_symlink() {
            continue;
        } else if !file.file_name().to_str().unwrap().ends_with(".rs") {
            continue;
        }

        let metadata = file.metadata().unwrap();
        let mut modified = metadata.modified().unwrap();

        if latest_time >= &mut modified {
            continue;
        }

        *latest_time = modified;
        println!("A file was modified: {}", file.path().display());
        return true;
    }

    false
}
