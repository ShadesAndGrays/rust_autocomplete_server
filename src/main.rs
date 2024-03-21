use std::fs::{self, OpenOptions};

#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::io::prelude::*;


#[get("/")]
fn index() -> &'static str{
    "hello"
}

#[get("/add/<firstnumber>/<secondnumber>")]
fn add(firstnumber:i32,secondnumber:i32)->String{

    (firstnumber + secondnumber).to_string()
}

#[get("/search/<word>")]
fn search(word:&str) -> Json<Vec<String>>{
    let words = fs::read_to_string("./words.txt").unwrap();
    let mut closest:Vec<String> = vec![];
    for compare in words.split(':'){
        println!("{}",compare);
        if compare.contains(word){
            closest.push(compare.to_owned());
        }
    }
    Json(closest)
}
#[post("/insert/<word>")]
fn add_word(word:&str){
    let mut option = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open("./words.txt")
        .unwrap();

    if let Err(e) = write!(option,":{}",word){
        eprintln!("Couldn't append to file {}",e);
    }
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index])
        .mount("/math",routes![add])
        .mount("/complete",routes![search])
        .mount("/complete", routes![add_word])
}
