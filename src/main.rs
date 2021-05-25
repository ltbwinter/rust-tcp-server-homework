// bring these libraries into scope explicitly
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

// a sub-function for incoming connection/request success handling
fn handle_connection(mut stream: TcpStream) {
    // declare a mutable buffer variable with 1024 byte
    let mut buffer = [0 as u8; 1024];

    // read the incoming stream params one by one
    while match stream.read(&mut buffer) {
        // if the size matches > success handling
        Ok(size) => {
            // echo everything
            stream.write(&buffer[0..size]).unwrap();
            // return true to main function
            true
        }

        // if the size does not match > error handling
        Err(_) => {
            // create console log
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            // terminate the connection
            stream.shutdown(Shutdown::Both).unwrap();
            // return false to main function
            false
        }
    } {}
}

// a function which always runs first in every executable Rust program
fn main() {
    // keep listening to port 7878 to accept incoming connections/requests
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // create console log to indicate this function is working
    println!("Server listening to port 7878");

    // process the incoming connections/requests one by one
    for stream in listener.incoming() {
        match stream {
            // success handling: create console log and pass the handling to handle_connection function
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_connection(stream));
            }

            // error handling: create console log
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
