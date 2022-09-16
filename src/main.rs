pub mod p0 {
    const MAX_OPEN_HANDLERS: u8 = 8;
    static OPEN_HANDLER: AtomicU8 = AtomicU8::new(0);

    use std::{
        io::{ErrorKind, Read, Write},
        net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
        sync::atomic::{AtomicU8, Ordering::SeqCst},
    };

    type Result<T, E = std::io::Error> = std::result::Result<T, E>;

    pub struct EchoServer {
        listener: TcpListener,
    }

    impl EchoServer {
        pub fn new(bind_addr: impl ToSocketAddrs) -> Result<Self> {
            let listener = TcpListener::bind(bind_addr)?;

            Ok(Self { listener })
        }

        pub fn serve(self) {
            for client in self.listener.incoming() {
                if let Ok(client) = client {
                    if let Ok(peer_addr) = client.peer_addr() {
                        log::info!("[{peer_addr}] Incoming connection");

                        let open_handlers = OPEN_HANDLER.fetch_add(1, SeqCst);

                        if open_handlers < MAX_OPEN_HANDLERS {
                            log::info!("[{peer_addr}][H/{open_handlers:02}] Spinning up thread");

                            std::thread::spawn(move || {
                                let _ = handle_client(open_handlers, peer_addr, client).unwrap();
                                let _ = OPEN_HANDLER.fetch_sub(1, SeqCst);
                            });
                        } else {
                            let _ = OPEN_HANDLER.fetch_sub(1, SeqCst);
                        }
                    }
                }
            }
        }
    }

    fn handle_client(id: u8, peer_addr: SocketAddr, mut conn: TcpStream) -> Result<()> {
        let mut buf = [0u8; 1024];

        // Read to end
        loop {
            log::info!("[{peer_addr}][H/{id:02}] Reading");
            match conn.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    log::info!("[{peer_addr}][H/{id:02}] Writing {n} bytes back");
                    conn.write_all(&buf[..n]).unwrap();
                }
                Err(e) if matches!(e.kind(), ErrorKind::Interrupted | ErrorKind::WouldBlock) => {
                    continue
                }
                Err(e) => return Err(e),
            };
        }

        log::info!("[{peer_addr}][H/{id:02}] Closing");
        conn.shutdown(std::net::Shutdown::Both).unwrap();
        Ok(())
    }

    pub fn run() -> Result<()> {
        const DESCRIPTION: &str = r#"
Running Problem #0: Smoke Test

A tcp server is running on port 8989.
Try to connect to it by running `nc -vt 127.0.0.1 8989`.
"#;

        println!("{}", DESCRIPTION);

        let server = EchoServer::new(("0.0.0.0", 8989))?;
        server.serve();

        Ok(())
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<_> = std::env::args().collect();
    let problem = args.get(1).expect("No problem selected");

    match problem.as_str() {
        "0" => {
            p0::run().unwrap();
        }
        _ => {
            panic!("Invalid problem: {problem}");
        }
    }
}
