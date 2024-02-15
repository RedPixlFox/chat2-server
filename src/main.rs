mod ascii_codes;

use std::{
    io::{stdin, Read},
    net::{SocketAddr, TcpListener, TcpStream},
    thread::{self, JoinHandle},
};

use crate::ascii_codes::style::*;

fn main() {
    println!("{BOLD}--- Server ---{BOLD_RESET}");

    // defining
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = match TcpListener::bind(addr) {
        Ok(tcp_listener) => tcp_listener,
        Err(e) => {
            println!("{e}");
            let _ = stdin().read_line(&mut String::new());
            return;
        }
    };
    let addr = listener.local_addr().unwrap();
    let mut handles: Vec<JoinHandle<()>> = vec![];

    println!("{DIM}bound to {DIM_RESET}{addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handles.push(thread::spawn(|| handle_connection(stream))),
            Err(e) => println!("{e}"),
        }
    }

    println!("{BOLD}{BLINK}--- Closed ---{BLINK_RESET}{BOLD_RESET}");
    let _ = stdin().read_line(&mut String::new());
}

fn handle_connection(mut stream: TcpStream) {
    println!("{DIM}{:?} connected{DIM_RESET}", stream);

    // let mut buf: Vec<u8> = vec![];
    // stream.read_to_end(&mut buf);

    println!("{DIM}{:?} disconnected{DIM_RESET}", stream);
}
