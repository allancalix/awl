use clap::clap_app;

use std::sync::Arc;
use std::sync::mpsc::channel;
use std::net::{UdpSocket, SocketAddr};

#[derive(Debug)]
struct SocketPair {
    source: SocketAddr,
    dest: SocketAddr,
}

fn main() -> std::io::Result<()> {
        let matches = clap_app!(awl_client =>
            (version: "0.1.0")
            (author: "Allan Calix <allan@acx.dev>")
            (@arg client_id: <CLIENT_ID> "The ID to register for the client node.")
            (@arg dest_id: <DESTINATION_ID> "The ID of the destination node.")
        )
        .get_matches();

        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0")?);
        let coordination_server = SocketAddr::from(([127, 0, 0, 1], 3050));

        let (sender, receiver) = channel();
        let recv = socket.clone();
        std::thread::spawn(move || {
            loop {
                let mut buf = [0; 4096];
                let (amt, src) = recv.recv_from(&mut buf).unwrap();
                println!("Read {} bytes from source {}", amt, src);
                let msg = String::from_utf8(buf[0..amt].into()).unwrap();
                let mut parts = msg.split_ascii_whitespace();

                let source_addr = parts.next().unwrap().parse().unwrap();
                let dest_addr = parts.next().unwrap().parse().unwrap();

                sender.send(SocketPair{
                    source: source_addr,
                    dest: dest_addr,
                }).unwrap();

                break;
            }
        });

        std::thread::sleep(std::time::Duration::from_secs(5));

        let message = format!("{} {}", matches.value_of("client_id").unwrap(), matches.value_of("dest_id").unwrap());
        socket.send_to(message.as_bytes(), coordination_server)?;

        let addr = receiver.recv().unwrap();
        println!("{:?}", addr);

        let (sender, receiver) = channel();
        let recv = socket.clone();
        std::thread::spawn(move || {
            let mut buf = [0; 4096];
            let (amt, src) = recv.recv_from(&mut buf).unwrap();

            println!("Got message: {} from {}", String::from_utf8(buf[0..amt].into()).unwrap(), src);

            sender.send(()).unwrap();
        });

        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if let Ok(_) = receiver.try_recv() {
                break;
            }

            socket.send_to("hello, world!".as_bytes(), addr.dest).unwrap();
        }

        Ok(())
}
