extern crate regex;
extern crate hyper;

use net::*;
use std::env;

mod net;

fn main() {

    // Get input from CLI
    // `env::args()` return `Args` struct
    // `Args` implement `Iterator` trait methods
    let url = env::args().nth(1);
    match url {
    	Some(val) => {
      for downloader in list_of_downloaders() {
  				match downloader.can_handle(&val) {
  					true => println!("URL: {} \nDetected by {}", val, downloader.id()),
  					false => continue
  				}
  			}
  		},

  		None => panic!("Ahhhh")

  	}


}

fn list_of_downloaders() -> Vec<Box<Downloader>> {

		let list: Vec<Box<Downloader>> = vec![
			Box::new(YouTube),
			Box::new(Vimeo),
		];
		list

}
