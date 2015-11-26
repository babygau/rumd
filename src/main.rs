extern crate regex;
extern crate hyper;

use net::*;
use std::env;
use std::process::exit;

mod net;

fn main() {

    // Get input from CLI
    // `env::args()` return `Args` struct
    let url = env::args().nth(1).unwrap_or("Bad news! I forget to send link to program\nI am well aware that I will have to do it again.\n".to_string());

    println!("URL: {}", url);

    let downloader: Box<Downloader>;

    match () {

      _ if YouTube::is_link_valid(&url) => downloader = Box::new(YouTube) as Box<Downloader>,

      _ if Vimeo::is_link_valid(&url) => downloader = Box::new(Vimeo) as Box<Downloader>,

      _ => {

        println!("Bad news! I need to double check my link, it appears that link is invalid");
        exit(0); // Exit the program

      },
    };

    println!("Good news, link will now be processed by: {}", downloader.name());


}


