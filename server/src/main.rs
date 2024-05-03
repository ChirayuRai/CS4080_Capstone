use std::{
    // Lets us read file contents to strings.
    fs,

    // Lets us read from and write to stream.
    io::{prelude::*, BufReader},

    // Library to help listen for connections
    net::{TcpListener, TcpStream},
};

fn main() {
    // "bind" acts like new, holding onto ip and port. unwrap stops the program if errors occur.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for loops calls handle_connection function, which asks for the current stream.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Wraps a mutable instance to the stream.
    let buf_reader = BufReader::new(&mut stream);
    // request_line gets the GET request from buf_reader.
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Checks request line, if the url is not modified, go through hello.html, otherwise print error.
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./404.html")
    };

    // Gets contents of respective html file.
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Formats the contents of html as well as the length.
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Writes the response down to the connection, displaying the html.
    stream.write_all(response.as_bytes()).unwrap();
}
