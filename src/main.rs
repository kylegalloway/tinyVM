extern crate tiny_vm;
extern crate clap;

use clap::{App, Arg};

fn main()
{

    let args = App::new("TinyVM")
        .about("Runs a tiny virtual machine")
        .version("0.0.1")
        .author("Kyle Galloway")
        .arg(Arg::with_name("file")
                 .help("the program file to use")
                 .index(1)
                 .required(true)
                 .short("f")
                 .long("file"))
        .get_matches();

    let program = args.value_of("file").unwrap();


    tiny_vm::main(String::from(program));
}
