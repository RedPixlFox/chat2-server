mod ascii_codes;

use std::{
    io::{stdin, BufRead, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
    thread::{self, sleep, JoinHandle},
    time::Duration,
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

    //stream.set_write_timeout(Some(Duration::from_millis(1)));

    let mut buf_reader = BufReader::new(&stream);

    let mut counter: usize = 0;
    loop {
        let mut buf = String::new();
        let _ = buf_reader.read_line(&mut buf);

        if buf == "" {
            break;
        } // break loop, if stream sends no bytes anymore

        if buf.trim() == "" {
            println!("{}", buf.replace("\r", "\\r").replace("\n", "\\n"));
            sleep(Duration::from_millis(1));
            continue;
        } // continue loop, because no data was sent

        println!(
            "[{counter}]: {}",
            buf.replace("\r", "\\r").replace("\n", "\\n")
        );

        counter += 1;
    }

    println!("{DIM}{:?} disconnected{DIM_RESET}", stream);
}
