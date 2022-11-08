use std::io::{ prelude::*, BufReader};
use std::fs::File;
use std::io::BufWriter;
use std::env;
use std::process;
use std::path::Path;
use http::{Request, Response};
mod working_status;
mod wto;
mod E3;
mod overtimes;


fn main() {
    //Get agruments from terminal
    let args: Vec<String> = env::args().collect();
    //TODO: check if agrs count unexeptable 

    //TODO: Handle unwrap
    let oparetion = args[1].parse::<i32>().unwrap();

    let file_path = &args[2];
    if !Path::new(file_path).exists() {
        
        //if file doesn't exist reply error to legacy app and exit
        reply(file_path, Err(String::from("Το συννημένο αρχείο δεν υπάρχει")));
        process::exit(1)}



    match oparetion {
        1 => {

        },
        2 => {

        },
        3 => {

        },
        _ => {

        },
        
    }
}

fn reply(file_name: &String, response: Result<String, String>){

}
