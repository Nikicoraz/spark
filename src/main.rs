use std::net::UdpSocket;
use std::io::{stdin, stdout, Write};
use std::process::exit;
use regex::Regex;

/// The magic packet is a frame that is most often
/// sent as a broadcast and that contains anywhere
/// within its payload 6 bytes of all 255 (FF FF FF FF FF FF in hexadecimal),
/// followed by sixteen repetitions of the target
/// computer's 48-bit MAC address, for a total of 102 bytes

fn main() {
    let mac_regex = Regex::new(r"([a-f0-9]{2}:){5}[a-f0-9]{2}").unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Cannot open the UDP socket");
    socket.set_broadcast(true).expect("Could not set the socket type to broadcast");
    socket.connect("255.255.255.255:0").expect("Could not set broadcast destination");

    println!("Sending broadcast from port {}", socket.local_addr().expect("Could not get UDP socket").port());

    print!("Enter device to wake MAC address (aa:bb:cc:dd:ee:ff): ");
    let _ = stdout().flush();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error while reading input");

    let mac = mac_regex.find(&input);
    match mac {
        None => {
            println!("Invalid MAC address");
            exit(1);
        },
        Some(mac) => {
            let six_ff = [0xffu8; 6];

            let mac = mac.as_str();
            let single_bytes = mac.split(":");

            let array: Vec<_> = single_bytes.map(|x| {x}).collect();
            let mut to_send: Vec<u8> = Vec::from_iter(six_ff);
            for _ in 0..16 {
                for byte in &array {
                    to_send.push(u8::from_str_radix(byte, 16).expect("Error during mac parsing"));
                }
            }


            println!("Sending bytes {:x?}", to_send);
            socket.send(&to_send).expect("Could not send the packet");

            println!("The device should wake now!");
        }
    }
}
