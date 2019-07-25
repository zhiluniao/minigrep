use std::env;
use std::process;
use minigrep;
fn main() {
    let args = env::args();
    let result = minigrep::Config::new(args);

    let _result = match result {
        Ok(_config) => {
            println!("Searching for {}", _config.query);
            println!("In file {}", _config.filename);

            if let Err(e) = minigrep::run(_config) {
                println!("Application error: {}", e);
                process::exit(1);
            }
        }
        Err(_error) => {
            panic!("There are some errors")
        }
    };
}

