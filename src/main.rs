use clap::{arg, App, Command};

fn main() {
    let cmd = App::new("mason")
        .bin_name("mason")
        .about("Mason CLI")
        .author("Pedro Luz <pedromsluz@gmail.com>")
        .version("0.1.0")
        .subcommand_required(true)
        .subcommand(
            Command::new("new")
                .about("Create a new block")
                .arg_required_else_help(true)
                .arg(arg!(<block_name> "The name of the block")),
        );
    let matches = cmd.get_matches();

    println!("{:#?}", matches);
}
