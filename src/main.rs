use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

/// Move files to Trash.
#[derive(StructOpt, Debug)]
#[structopt(name = "wrap")]
struct Opt {

    /// Files to process
    #[structopt(name = "FILES", parse(from_os_str), required = true)]
    files: Vec<PathBuf>,


    /// Destination folder
    #[structopt(name = "DEST", required = true)]
    dest: String,
}

fn wrapped_main(opt: Opt) -> Result<(), String>{
    // Check if all files exist
    for file in &opt.files {
        if !file.exists() {
            return Err(format!("File {:?} does not exist", file));
        }
    }
    // Create destination directory
    let err = std::fs::create_dir_all(&opt.dest);
    if err.is_err() {
        return Err(format!("Could not create directory {:?}: {}", opt.dest, err.unwrap_err()));
    }
    // Move files
    for file in opt.files {
        let mut dest = PathBuf::from(&opt.dest);
        dest.push(file.file_name().unwrap());
        let err = std::fs::rename(&file, dest);
        if err.is_err() {
            return Err(format!("Could not move file {:?} to {:?}: {}", file, opt.dest, err.unwrap_err()));
        }
    }
    return Ok(());
}
fn main() {
    let opt = Opt::from_args();
    match wrapped_main(opt) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
