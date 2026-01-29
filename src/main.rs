use crate::client::gui::user_interface::client_main;
use crate::server::server_main::server_main;
use std::io;

mod client;
mod common;
mod server;

enum LaunchType {
    Server,
    Client,
}

fn input_launch_type() -> LaunchType {
    let mut input = String::new();
    println!("Please type what you want to launch (Server/Client): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let formated_input = input.trim().to_lowercase();
    if formated_input == "server" {
        LaunchType::Server
    } else if formated_input == "client" {
        LaunchType::Client
    } else {
        input_launch_type()
    }
}

#[tokio::main]
pub async fn main() -> iced::Result {
    match input_launch_type() {
        LaunchType::Client => client_main(),
        LaunchType::Server => {
            let server = server_main();
            server.await;
            Ok(())
        }
    }
}
