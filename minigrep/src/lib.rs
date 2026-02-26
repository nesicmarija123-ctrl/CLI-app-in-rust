use std::fs;
use std::error::Error;

//everything in rust is private by default

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    println!("With text: \n{}", contents);
    Ok(())
}

pub struct Config{
    pub query: String,
    pub filename: String,
}

impl Config{
    //umesto parse_config nazvacemo je new, to je konvencija za construct functions
 //   fn new(args: &[String]) -> Config{
 //bolje je da se vrati Result koji ce imati opciju da vrati Config ili string ukoliko se desi error
    pub fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }

        let query =  args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})    
    }

}