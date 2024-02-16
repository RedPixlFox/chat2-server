#![allow(unused)]

mod ascii_codes;

use std::{
    io::{stdin, BufRead, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
    os::windows::io::{AsRawSocket, AsSocket},
    thread::{self, panicking, JoinHandle},
};

use crate::ascii_codes::{color::*, style::*};

fn main() {
    println!("{BOLD}--- Server ---{BOLD_END}");

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

    println!("{DIM}bound to {DIM_END}{addr}");

    let mut stream_counter: u128 = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_id = stream_counter;
                stream_counter += 1;
                handles.push(thread::spawn(move || handle_connection(stream, stream_id)));
            }
            Err(e) => println!("{e}"),
        }
    }

    println!("{BOLD}{BLINK}--- Closed ---{BLINK_END}{BOLD_END}");
    let _ = stdin().read_line(&mut String::new());
}

fn handle_connection(mut stream: TcpStream, stream_id: u128) {
    println!(
        "{F_GREEN}+{F_DEFAULT} {DIM}TcpStream {} id: {stream_id}, peer: {}, socket: {} {}{DIM_END}",
        r#"{"#,
        stream.peer_addr().unwrap().to_string(),
        stream.as_raw_socket().to_string(),
        r#"}"#
    );

    //stream.set_write_timeout(Some(Duration::from_millis(1)));

    let mut buf_reader = BufReader::new(&stream);

    let mut username: String = stream_id.to_string();
    let mut counter: usize = 0;
    loop {
        let mut buf = String::new();
        let _ = buf_reader.read_line(&mut buf);

        if buf == "" {
            break;
        } // break loop, if stream sends no bytes anymore

        // handle collected data:        
        let mut package: Vec<&str> = buf.split('\r').collect();
        //println!("[{stream_id}]: {:?}", package);
        process_package(package, stream_id, &mut username);

        counter += 1;
    }

    println!(
        "{F_RED}-{F_DEFAULT} {DIM}TcpStream {} id: {stream_id}, peer: {}, socket: {} {}{DIM_END}",
        r#"{"#,
        stream.peer_addr().unwrap().to_string(),
        stream.as_raw_socket().to_string(),
        r#"}"#
    );
}

/// returns a response for the client
fn process_package<'a>(mut package: Vec<&'a str>, stream_id: u128, mut username: &'a mut String) -> Vec<&'a str> {
    if package.len() > 0 {
        match package[0] {
            "send_msg" => {
                if package.len() > 2 {
                    for i in 1..package.len()-1 {
                        println!("{DIM}[{username}]:{DIM_END} {}", package[i]);
                    }
                    vec!["0"]
                } else {
                    vec!["1", "missing args: [message/s]"]
                }
            }
            "set_username" => {
                if package.len() == 3 {
                    *username = package[1].to_string();
                    vec!["0"]
                } else {
                    vec!["1", "missing arg: [username]"]
                }
            }
            // "get_username" => {
            //     vec!["0", "username"]
            // }
            _ => {
                todo!("Handle unknown commands")
            }
        }
    } else {
        vec!["1", "missing args: [message/s]"]
    }
}
