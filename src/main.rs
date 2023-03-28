use std::ptr::null;
use clap::Parser;

use crate::chat::server::*;
use crate::chat::client::*;
mod chat;

#[derive(Parser)]
struct Cli {
    instance: String,
}

fn main() {
    let args = Cli::parse();
    if args.instance != "server"{
        start_pu_client()
    }

    println!("{}", args.instance);
    start_pu_server()
}
