use std::error::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpSocket, TcpStream};
use wordle::msg::{ClientMsg, ServerMsg};
use wordle::words::{GuessState, Words};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:4321".parse().unwrap();
    let socket = TcpSocket::new_v4()?;
    socket.bind(addr)?;

    let listener = socket.listen(1024)?;
    let words = Words::new();

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                tokio::spawn(handle_stream(socket, addr, words.clone()));
            }
            Err(e) => {
                eprintln!("error occured: {:?}", e)
            }
        }
    }

    Ok(())
}

async fn handle_stream(mut socket: TcpStream, _addr: SocketAddr, words: Words) {
    let mut answer = words.generate_answer();
    let user = format!(
        "{}{:04}",
        words.show(&answer).0,
        rand::random_range(0..10000)
    );

    loop {
        let mut buffer = vec![0; 1024];
        let n = socket.read(&mut buffer).await.unwrap();
        let guess = ClientMsg::from_slice(&buffer[..n]).guess();

        if guess == "/ABORT" {
            eprintln!("user aborted.");
            return;
        }

        eprintln!("{}: {}", user, guess);

        let guess_result = words.solve(&mut answer, &guess);
        let tries = words.show(&answer).1;

        let mut exit = false;

        let response = match guess_result {
            Ok((guess_state, colors)) => {
                match guess_state {
                    GuessState::Solved => {
                        eprintln!("{}: Solved!", user);
                        exit = true;
                    },
                    GuessState::GameOver => {
                        eprintln!("{}: Game Over!", user);
                        exit = true;
                    }
                    _ => {}
                };
                ServerMsg::new(guess_state, tries, colors)
            }
            Err(e) => ServerMsg::new(wordle::words::GuessState::Error(e), tries, vec![]),
        };

        socket
            .write_all(response.to_string().as_bytes())
            .await
            .unwrap();

        if exit {
            return;
        }
    }
}
