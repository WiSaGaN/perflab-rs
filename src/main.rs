use std::os::unix::net::UnixDatagram;

const NUM_ITERATIONS: usize = 100_000;
const BUFFER_SIZE: usize = 512;

fn main() {
    let (server, client) = UnixDatagram::pair().unwrap();
    let thread_client = std::thread::spawn(move || {
        let mut buffer = [0u8; BUFFER_SIZE];
        let size = client.recv(&mut buffer).unwrap();
        client.send(&buffer[0..size]).unwrap();
        for _ in 0..NUM_ITERATIONS {
            let size = client.recv(&mut buffer).unwrap();
            client.send(&buffer[0..size]).unwrap();
        }
    });
    let mut buffer = [0u8; BUFFER_SIZE];
    server.send(b"warm up").unwrap();
    let _ = server.recv(&mut buffer).unwrap();
    let start = std::time::Instant::now();
    for _ in 0..NUM_ITERATIONS {
        server.send(&buffer[0..(BUFFER_SIZE - 10)]).unwrap();
        let _ = server.recv(&mut buffer).unwrap();
    }
    let end = std::time::Instant::now();
    let duration = end - start;
    let latency = duration / NUM_ITERATIONS as u32;
    println!("latency = {:?}", latency);
    thread_client.join().unwrap();
}
