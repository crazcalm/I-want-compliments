use I_want_compliments::db;

use std::env;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: Option<String>,

    #[arg(short, long)]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("path: {:?}", cli.path.unwrap_or(".".to_string()));
    println!("dry_run: {:?}", cli.dry_run);

    println!("Howdy");

    for argument in env::args() {
        println!("{argument}");
    }

    let conn = match cli.dry_run {
        true => db::get_in_memory_connection().expect("unable to create in memory database"),
        false => db::get_connection().expect("Unable to create database"),
    };

    db::initialize_db(conn).expect("Unable to intialize the database");
}
