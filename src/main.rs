use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
fn main() {
    //TCP Listener for the address to recive a connection
    let listener =
        TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        // call to command that handless what happens during a conntection
        handle_connection(stream)
    }
}

//Method to handle connections
fn handle_connection(mut stream: TcpStream){

    //buffer to handle the size of packets
    let mut buffer = [0; 1024];

    //read through the connection packet
    stream.read(&mut buffer).unwrap();

    //variable used to validation the connection buffer
    let get = b"GET / HTTP/1.1\r\n";

    //tuple variable that will return the buffer status as well the file that goes with it
    let (status_line, filename) =
        if buffer.starts_with(get)
        {
            ("HTTP/1.1 200 OK", "index.html")
        }
        else{
            ("HTTP/1.1 404 Not Found", "404.html")
        };

    // variabble that allows rust to grab the index file or error page file
    let contents = 
        fs::read_to_string(filename).unwrap();
    
    //variable used to read the file and place it on the web page
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
      status_line,
      contents.len(),
      contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
