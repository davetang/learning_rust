// this file should be included in the src directory
// after running cargo new
//
// [dependencies]
// clap = "2.33"

// import the clap::App struct
use clap::{App, Arg};

fn main() {
    // create a new App
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Dave Tang <me@davetang.org>")
        .about("Implementation of echo using Rust")
        .arg(
            // create new Arg called text
            Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1)
            )
        .arg(
            // create new Arg called omit_newline
            Arg::with_name("omit_newline")
            .long("omit_newline")
            .short("n")
            .help("Do not print newline")
            .takes_value(false)
            )
        // tells the App to parse the args
        .get_matches(); 

    // pretty print for debugging
    // println!("{:#?}", matches);

    // only use unwrap if we are sure that
    // the value will not be None
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    // join will insert a str between all elements of a
    // vector (in this case all the command args)
    // if is an expression in Rust, which means it returns a value
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
