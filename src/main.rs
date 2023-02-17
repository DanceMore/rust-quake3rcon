use std::env;
use std::net::{ToSocketAddrs, UdpSocket};

fn main() -> std::io::Result<()> {
    // default values
    let mut host = "localhost";
    let mut port = 27960;
    let mut rconpass = "password";
    let mut buf = [0u8; 2084];

    // parse command-line arguments
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        match args[i].as_str() {
            "--host" => {
                host = &args[i + 1];
            }
            "--port" => {
                port = args[i + 1].parse().unwrap();
            }
            "--rconpass" => {
                rconpass = &args[i + 1];
            }
            _ => (),
        }
    }

    let addr = (host, port).to_socket_addrs()?.next().unwrap();

    // packet header
    let header = b"\xff\xff\xff\xffrcon ";

    // open socket to speak to server
    let udp_sock = UdpSocket::bind("0.0.0.0:0")?;

    // the meat
    let command = args.last().unwrap().as_str();
    let mut data_out = Vec::new();
    data_out.extend_from_slice(header);
    data_out.extend_from_slice(rconpass.as_bytes());
    data_out.push(b' ');
    data_out.extend_from_slice(command.as_bytes());
    if udp_sock.send_to(&data_out, &addr).is_ok() {
        println!("{}", String::from_utf8_lossy(&data_out));
    }

    let (amt, _src) = udp_sock.recv_from(&mut buf)?;
    let data_in = &buf[..amt];
    if data_in.is_empty() {
        println!("no response from server");
    } else {
        println!("{}", String::from_utf8_lossy(data_in));
    }

    // Close socket
    Ok(())
}
