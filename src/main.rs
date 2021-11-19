use std::env;
use std::io;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "f", long = "config-file")]
    config_file: Option<String>,
}

fn log_file_path() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in some directory");
    let mut dir = dir.join("");
    dir.pop();
    dir.pop();
    dir.push("logs");
    if !dir.as_path().is_dir() {
        if dir.as_path().exists() {
            panic!(
                r#"The file path used for logs {} is not a directory. Please resolve conflict."#,
                dir.display());
        } else {
            fs::create_dir(&dir)?;
        }
    }

    dir.push("rshot.log");

    Ok(dir)
}

fn config_file_path() -> io::Result<PathBuf> {
    let opts = Opt::from_args();

    match opts.config_file {
        None => {
            let exe = env::current_exe()?;
            let dir = exe.parent().expect("Executable must be in some directory");
            let mut dir = dir.join("");
            dir.pop();
            dir.pop();
            dir.push("config.toml");
            Ok(dir)
        },
        Some(ref config_file) => {
            let path = fs::canonicalize(config_file);
            match path {
                Ok(ref path) => {
                    println!("config file canonicalized path = {}",
                            path.display());
                },
                Err(ref e) =>
                    println!("oops got error = {:?} calling canonicalize on={}",
                        e, config_file),
            }
            path
        }
    }
}

fn main() {
    let pathbuf = config_file_path().expect("Couldn't");
    println!("config file = {}", pathbuf.display());

    let pathbuf = log_file_path().expect("Couldn't");
    println!("log file = {}", pathbuf.display());
}
