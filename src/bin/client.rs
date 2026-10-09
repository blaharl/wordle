use std::error::Error;
use std::io::prelude::*;
use std::net::TcpStream;

use wordle::msg::{ClientMsg, ServerMsg};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:4321")?;

    loop {
        let mut buffer = vec![0; 1024];
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).ok();
        guess = guess.trim().to_string().to_ascii_uppercase();

        let client_msg = ClientMsg::new(guess.clone());
        stream.write_all(client_msg.to_string().as_bytes())?;

        if guess == "/ABORT" {
            break;
        }

        let n = stream.read(&mut buffer)?;
        let server_msg = ServerMsg::from_slice(&buffer[..n]);
        println!(
            "{:?} {:?} {:?}",
            server_msg.colors(),
            server_msg.guess_state(),
            server_msg.tries(),
        );

        match server_msg.guess_state() {
            wordle::words::GuessState::GameOver | wordle::words::GuessState::Solved => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
