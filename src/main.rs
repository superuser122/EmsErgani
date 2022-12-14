use std::fs;
use std::env;
use std::fs::File;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::path::PathBuf;
use std::process;
use std::path::Path;
mod working_status;
mod wto;
mod anaggelia;
mod overtimes;
mod card;
mod http_client;
use convert::get_cards;
use http_client::*;
mod credentials;
mod auth_response;
use convert::{get_anaggelia, get_working_status, get_wtos, get_overtimes};
mod convert;
use std::io::Read;

use encoding_rs::WINDOWS_1253;
use encoding_rs_io::DecodeReaderBytesBuilder;



fn main() {
    //Get agruments from terminal
    let args: Vec<String> = env::args().collect();
    //println!("args: {} {}", args[0],args[1]);
    //pause();
    if args.len() != 2 {
        //println!("exit args.len() != 2");
        process::exit(1)
    }

    let file_path = &args[1];
    //println!("file_path: {}", file_path);
    //pause();

    if !Path::new(file_path).exists() {
        
        //if file doesn't exist reply error to legacy app and exit
        reply(file_path, Err(String::from("Το συννημένο αρχείο δεν υπάρχει")));
        //println!("path not exists {}", file_path);
        process::exit(1)
    }
    //encoding
    let mut nfile = String::new();
    let file = File::open(file_path).unwrap();
    let mut rdr = DecodeReaderBytesBuilder::new()
    .encoding(Some(WINDOWS_1253))
    // .encoding(Some(encoding_rs::UTF_8))
    .build(file);  
    rdr.read_to_string(&mut nfile).unwrap();
    //let response_path = format!("{}{}", file_path, );

    fs::write(file_path,nfile).ok();
    //println!("encoding filepath: {}", file_path);
    //pause();


    let mut user_name = String::new();
    let mut password = String::new();
    let mut user_type = String::new();

                                             
    match fs::read_to_string(file_path) {       
        Ok(contents) => {
            if let Some(first_line) = contents.lines().next() {
                let line = first_line.to_string();
                let cells: Vec<&str> = line.split(";").collect();
                if cells.len() < 5  {
                    reply(file_path, Err(String::from("Η κεφαλίδα αρχείου έχει μη αποδεκτό πλήθος κολόνων")));
                    process::exit(1)
                }
                if cells[0] != "0"  {
                    reply(file_path, Err(String::from("Δεν υπάρχει γραμμή κεφαλίδας αρχείου")));
                    process::exit(1)
                }
                user_name = cells[2].to_string();
                password = cells[3].to_string();
                user_type = cells[4].to_string();
                
            }

            match get_path_and_body(contents) {
                Ok(path_body) => {
                    let auth_client = AuthClient::new(user_name, password, user_type);
                    match auth_client.get_token(){
                        Ok(token) => {
                            let res = HttpClient::new()
                                .url(path_body.0)
                                .token(token)
                                .body(path_body.1)
                                .build()
                                .send();
                            reply(file_path, res);
                            
                        },
                        Err(e) => {
                            reply(file_path, Err(e));
                            process::exit(1)
                        }
                    }
                },
                Err(e) => {
                    reply(file_path, Err(e));
                    process::exit(1)
                }
            }
        },       
        Err(_) => {
            reply(file_path, Err(String::from("Πρόβλημα ανάγνωσης αρχείου")));
            process::exit(1)
        }
    }

}



fn reply(file_name: &String, response: Result<(), String>){
   let no_ext =  Path::new(file_name).file_stem().unwrap_or_else(|| {process::exit(1)}).to_str().unwrap_or_else(|| {process::exit(1)});
   
   let path = PathBuf::from(file_name);
   let dir = path.parent().unwrap();

   let response_path = format!("{}{}",no_ext,".ans");

   
    
   let text = match response {
        Ok(_) => {
            "0;".to_string()
        },
        Err(e) => {
            format!("{}{}", "1;", e)
        },
   };


    //println!("reply path: {}", path.to_str().unwrap());
    //pause();
    //println!("reply dir: {}", dir.to_str().unwrap());
    //pause();
    //println!("reply file_name: {}", file_name);
    //pause();
    //println!("reply no_ext: {}", no_ext);
    //pause();
    //println!("reply new_path: {}", new_response_path.to_str().unwrap());
    //pause();

    // let mut nfile = String::new();
    // let file = File::open(file_name).unwrap();
    // let mut rdr = DecodeReaderBytesBuilder::new()
    // .encoding(Some(EUC_JP))
    // // .encoding(Some(encoding_rs::UTF_8))
    // .build(file);  
    // rdr.read_to_string(&mut nfile).unwrap();
    // //let response_path = format!("{}{}", file_path, );
    // println!("FILE: {}", nfile);
    // pause();   

    let new_response_path = dir.join(response_path);

   fs::write(new_response_path, text).unwrap_or_else(|_| {process::exit(1)});
}

fn get_path_and_body(contents: String) -> Result<(String, String), String> {
    if let Some(first_line) = contents.lines().next() {
        let line = first_line.to_string();
        let cells: Vec<&str> = line.split(";").collect();
        if cells.len() < 2  { return Err("Η κεφαλίδα αρχείου έχει μη αποδεκτό πλήθος κολόνων".to_string());}
        if cells[0] != "0"  { return Err("Δεν υπάρχει γραμμή κεφαλίδας αρχείου".to_string());}
        let lines = contents.lines().collect();
        match cells[1] {
            "1" => {
                let anaggelia = get_anaggelia(lines)?;
                let body = serde_json::to_string(&anaggelia).map_err(|e| e.to_string())?;
                let path = "E3".to_string();
                return Ok((path, body));
            },
            "2" =>{
                let working_status = get_working_status(lines)?;
                let body =  serde_json::to_string(&working_status).map_err(|e| e.to_string())?;
                let path = "WKChgWK".to_string();
                return Ok((path, body));

            },
            "3" => {
                let wto_week = get_wtos(lines)?;
                let body =  serde_json::to_string(&wto_week).map_err(|e| e.to_string())?;
                let path = "WTOWeek".to_string();
                return Ok((path, body));

            },
            "4" => {
                let wto_week = get_wtos(lines)?;
                let body =  serde_json::to_string(&wto_week).map_err(|e| e.to_string())?;
                let path = "WTODaily".to_string();
                return Ok((path, body));

            },
            "5" => {
                let overtime = get_overtimes(lines)?;
                let body = serde_json::to_string(&overtime).map_err(|e| e.to_string())?;
                let path = "OvTime".to_string();
                return Ok((path, body));
            },
            "6" => {
                let cards = get_cards(lines)?;
                let body = serde_json::to_string(&cards).map_err(|e| e.to_string())?;
                let path = " WRKCardSE".to_string();
                return Ok((path, body));

            },
            _ => {
                return Err("Άγνωστος τύπος αρχείου".to_string());
            }
            
        };

    };
    Err("Ελέγξτε το αρχείο".to_string())
}


fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}


//Unit test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_anaggelia_test() {

        let contents = fs::read_to_string("E3test.txt").unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        let anaggelia = get_anaggelia(lines).unwrap();

        let body = serde_json::to_string(&anaggelia).unwrap();

        println!("{}", body);

    }
}
