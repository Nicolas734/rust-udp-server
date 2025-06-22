use std::net::UdpSocket;


fn main() -> std::io::Result<()> {
    let socket: UdpSocket = UdpSocket::bind("127.0.0.1:34254")?;
    let mut buf: [u8; 10] = [0; 10];
    loop {
            let (amt, src) = socket.recv_from(&mut buf)?;
            println!("Recebido {} bytes de {}: {:?}", amt, src, String::from_utf8_lossy(&buf[..amt]));
            let buf_received: &mut [u8] = &mut buf[..amt];
            socket.send_to(&buf_received, &src)?;
    }
}