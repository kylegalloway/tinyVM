extern crate tiny_vm;
extern crate clap;

use clap::App;

fn main()
{

    App::new("TinyVM")
        .about("Runs a tiny virtual machine")
        .version("0.0.1")
        .author("Kyle Galloway");

    tiny_vm::main();
}
