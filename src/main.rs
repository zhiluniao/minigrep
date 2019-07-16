use std::env;
use std::process;

pub mod minigrep{

    use std::fs;
    use std::error::Error;
    use std::result::Result;

    pub struct Config{
        pub query:String,
        pub filename:String,
    }

    impl Config{
        pub fn new(args : &[String]) -> Result<Config,&'static str>{
            if args.len() < 3{
                return Err("Not enough arguments");
            }
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok( Config{ query, filename })
        }

        pub fn run(config:Config)-> Result<(),Box<dyn Error>>{
            let contents= fs::read_to_string(config.filename)?;
            println!("With text:\n{}",contents);
            Ok(())
        }
    }


}

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}",args);

//    let config = parse_config(&args);

    let result = minigrep::Config::new(&args);

    let _result = match result {
      Ok(_config) => {
          println!("Searching for {}",_config.query);
          println!("In file {}",_config.filename);

          if let Err(e) = minigrep::Config::run(_config){
              println!("Application error: {}",e);

              process::exit(1);
          }

      },
        Err(_error) =>{
            panic!("There are some errors")
        }
    };

}

