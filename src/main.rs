use minigrep;
use std::env;
use std::process;
fn main() {
    let args = env::args();
    let result = minigrep::Config::new(args);

    let _result = match result {
        Ok(config) => {
            println!("Searching for {}", config.query);
            println!("In file {}", config.filename);

            if let Err(e) = minigrep::run(config) {
                println!("Application error: {}", e);
                process::exit(1);
            }
        }
        Err(error) => panic!("There are some errors : {}", error),
    };
}
