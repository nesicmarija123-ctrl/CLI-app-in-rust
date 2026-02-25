use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    //vector of strings, because collect needs to know what type of collection we want
    let query =  &args[1]; // reference to our vector, nulti argument je binary path 
    let filename = &args[2];
   // println!("{:?}", args);
   println!("Searching for {}", query);
   println!("In file {}", filename);
}
