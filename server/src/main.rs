use core::time::Duration;
use server::ThreadPool;
use std::{
    // Lets us read file contents to strings.
    fs,
    // Lets us read from and write to stream.
    io::{prelude::*, BufReader},
    // Library to help listen for connections
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    // "bind" acts like new, holding onto ip and port. unwrap stops the program if errors occur.
    let server = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(server.clone()).unwrap();
    println!("Server running on url: {}", server);

    // Single threaded
    // // for loops calls handle_connection function, which asks for the current stream.
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    // }

    // Bad multi-threaded
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     thread::new(|| {
    //         handle_connection(stream);
    //     })
    // }

    // Good multi threaded
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shuttind down.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
