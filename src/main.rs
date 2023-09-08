extern crate clap;
use clap::{App, Arg};
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Define the command-line interface using clap
    let matches = App::new("RCON Client")
        .color(clap::ColorChoice::Auto)
        .arg(
            Arg::with_name("host")
                .long("host")
                .value_name("HOST")
                .default_value("localhost")
                .help("Server hostname or IP address"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .default_value("27960")
                .help("Server port"),
        )
        .arg(
            Arg::with_name("rconpass")
                .long("rconpass")
                .value_name("PASSWORD")
                .default_value("password")
                .help("RCON password"),
        )
        .arg(
            Arg::with_name("COMMAND")
                .help("RCON command to send")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Extract values from command-line arguments
    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();
    let rconpass = matches.value_of("rconpass").unwrap();
    let command = matches.value_of("COMMAND").unwrap();

    let addr = (host, port).to_socket_addrs()?.next().unwrap();

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
