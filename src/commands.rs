use std::net::TcpStream;
use std::env;

fn check_port(port: &str) -> bool {
    return TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok();
}

pub fn scan_port() {
    match env::args().nth(2) {
        Some(port) => {
            if check_port(&port) {
                println!("Port {} is open", port);
            }
            else {
                println!("Port {} is closed", port);
            }
        }
        None => println!("No port provided")
    }
}
