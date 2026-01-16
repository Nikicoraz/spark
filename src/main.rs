use std::net::UdpSocket;

/// The magic packet is a frame that is most often
/// sent as a broadcast and that contains anywhere
/// within its payload 6 bytes of all 255 (FF FF FF FF FF FF in hexadecimal),
/// followed by sixteen repetitions of the target
/// computer's 48-bit MAC address, for a total of 102 bytes

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Cannot open the UDP socket");
    socket.set_broadcast(true).expect("Could not set the socket type to broadcast");
    socket.connect("255.255.255.255:0").expect("Could not set broadcast destination");

    println!("Connected on port {}", socket.local_addr().expect("Could not get UDP socket").port());

    let buf = [0xffu8; 6];
    socket.send(&buf).expect("Could not send the packet");
}
