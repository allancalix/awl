use std::collections::HashMap;
use std::net::UdpSocket;

pub struct Server {
    clients: HashMap<String, String>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            clients: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:3050")?;

        loop {
            let mut buf = [0; 4096];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            println!("Read {} bytes", amt);

            let msg = String::from_utf8(buf[0..amt].to_vec()).unwrap();
            let mut parts = msg.split_ascii_whitespace();
            let client_id = parts.next().unwrap();
            let dest_client_id = parts.next().unwrap();

            self.clients.insert(client_id.to_string(), src.to_string());
            println!("{} connecting to dest {}", client_id, dest_client_id);

            if let Some(dst) = self.clients.get(dest_client_id) {
                let payload = format!("{} {}", src, dst);

                socket.send_to(payload.as_bytes(), &src).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
