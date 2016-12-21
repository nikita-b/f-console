use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read, BufReader, BufRead};


fn check_password(password: String) -> bool {
    let PASSWORD = String::from("12345");
    if password.trim() == PASSWORD {
        return true;
    }
    false
}


fn handle_client(mut stream: TcpStream) {
    let mut rstream = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();

    stream.write(b"Welcome to F-Console, Stranger!\nEnter password:").unwrap();
    rstream.read_line(&mut buffer).unwrap();
    match check_password(buffer) {
        false => {
            println!("Password correct!");
        }
        true  => {
            println!("Nope");
        }
    }
}

fn main() {


    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
             }
            Err(e) => { 
                println!("Connection failed: {}", e);
            }
        }
    }
}
