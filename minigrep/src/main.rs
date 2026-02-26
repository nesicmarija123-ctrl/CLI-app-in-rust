use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); 
  
    //let(query, filename) = parse_config(&args);
    //let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);


   if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);
        process::exit(1);
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