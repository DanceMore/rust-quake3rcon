use clap::{Arg, Parser};
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    host: String,

    #[clap(long, default_value = "27960")]
    port: u16,

    #[clap(long, default_value = "password")]
    rconpass: String,

    #[clap(name = "COMMAND")]
    command: String,
}

fn main() -> std::io::Result<()> {
    // Define the command-line interface using clap
    let args: Args = Args::parse();

    // Extract values from command-line arguments
    let host = &args.host;
    let port = args.port;
    let rconpass = &args.rconpass;
    let command = &args.command;

    // Convert host and port into a SocketAddr
    let addr = format!("{}:{}", host, port)
        .to_socket_addrs()?
        .next()
        .unwrap();

    // Packet header
    let header = b"\xff\xff\xff\xffrcon ";

    // Open socket to speak to the server
    let udp_sock = UdpSocket::bind("0.0.0.0:0")?;

    // Set read and write timeouts for the UDP socket
    let timeout_duration = Duration::from_secs(5); // Adjust this value as needed
    udp_sock.set_read_timeout(Some(timeout_duration))?;
    udp_sock.set_write_timeout(Some(timeout_duration))?;

    // Construct the RCON packet
    let mut data_out = Vec::new();
    data_out.extend_from_slice(header);
    data_out.extend_from_slice(rconpass.as_bytes());
    data_out.push(b' ');
    data_out.extend_from_slice(command.as_bytes());

    if udp_sock.send_to(&data_out, &addr).is_ok() {
        println!("{}", String::from_utf8_lossy(&data_out));
    }

    let mut buf = [0u8; 2084];
    if let Ok((amt, _src)) = udp_sock.recv_from(&mut buf) {
        let data_in = &buf[..amt];
        if data_in.is_empty() {
            println!("no response from server");
        } else {
            println!("{}", String::from_utf8_lossy(data_in));
        }
    } else {
        println!("socket operation timed out");
    }

    Ok(())
}
