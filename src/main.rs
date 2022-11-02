use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let handle_server = thread::spawn(move || {
        println!("[Server] : start ... ");
        worker_server();
        println!("[Server] : close ... ");
    });
    let handle_client = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        println!("[Client] : start ... ");
        worker_client();
        println!("[Client] : close ... ");
    });
    handle_client.join();
    handle_server.join();
}

fn worker_server() {
    let listener = TcpListener::bind("127.0.0.1:9528");
    if !listener.is_ok() {
        println!("[Server] : Bind ip and port fail ... ");
        return;
    }
    let listener = listener.unwrap();
    println!("[Server] : Waiting for next message ... ");
    for stream in listener.incoming() {
        if !stream.is_ok() {
            println!("[Server] : Getting error message ... ");
            continue;
        }
        let mut stream = stream.unwrap();
        process_stream(stream);
        println!("[Server] : Waiting for next message ... ");
    }
}

fn process_stream(mut stream: TcpStream) -> bool {
    let mut buffer = [0; 1024];//read request from TcpStream
    if !stream.read(&mut buffer).is_ok() {
        return false;
    }
    println!("[Server][process_stream] Get Request Info : \"{}\"", String::from_utf8_lossy(&buffer[..]));
    if !stream.write(b"Server has received your request ... ").is_ok() {
        return false;
    }
    return true;
}

fn worker_client() {
    let mut stream = TcpStream::connect("127.0.0.1:9528");
    if !stream.is_ok() {
        println!("[Cliend] : Connect fail ... ");
        return;
    }
    let mut stream = stream.unwrap();
    let status = stream.write(b"client send info to server !");
    if !status.is_ok() {
        println!("[Client] : Send info fail");
        return;
    }
    let mut buffer = [0; 1024];
    let status = stream.read(&mut buffer);
    if !status.is_ok() {
        println!("[Client] : Recv info fail ... ");
        return;
    }
    println!("[Client] : Get msg from server \"{}\"", String::from_utf8_lossy(&buffer[..]));
    stream.shutdown(Shutdown::Both);
}