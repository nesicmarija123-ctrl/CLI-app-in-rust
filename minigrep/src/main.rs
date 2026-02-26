use std::env;
//module to help us with module reading 
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); 
  
    //let(query, filename) = parse_config(&args);
    //let config = parse_config(&args);
    let config = Config::new(&args);


    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

   let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text: \n{}", contents);

}

struct Config{
    query: String,
    filename: String,
}

impl Config{
    //umesto parse_config nazvacemo je new, to je konvencija za construct functions
 //   fn new(args: &[String]) -> Config{
 //bolje je da se vrati Result koji ce imati opciju da vrati Config ili string ukoliko se desi error
    fn new(args: &[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }

        let query =  args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})    
    }

}

/* 
//args will be reference to the array of strings
//and it will return a tuple with 2 string slices
//fn parse_config(args: &[String]) -> (&str, &str){
//sad menjamo da bude jasno da su vezani, sad ce vratiti config type
fn parse_config(args: &[String]) -> Config{
    //extract out query and filename from the args parameter
 //   let query =  &args[1]; // reference to our vector, nulti argument je binary path 
 //   let filename = &args[2];
//ne poklapaju se tipovi, moramo proslediti stringove, ali onda posto necemo da uzimamo ownership, moramo clone
    let query =  args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
    //return a tuple
   // (query, filename)
}

//parse_config is very closely tied to our config struct but our program doesn't express this coupling
//to fix this instead of having a separate function to parse command line arguments 
//we can add this logic to the implementation of our config struct
//to do that let's create an implementation block for our config struct 

*/