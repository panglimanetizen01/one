use std::{fs, io::{Read, Write}, net::TcpListener, process::Command, thread, time::{SystemTime, UNIX_EPOCH}};

fn fixture(source: &str) -> std::path::PathBuf {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let path = std::env::temp_dir().join(format!("one-http-{id}.one"));
    fs::write(&path, source).unwrap();
    path
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_one")).args(args).output().unwrap()
}

#[test]
fn cli_http_get_executes_with_explicit_target_permission() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("HTTP test server should bind");
    let address = listener.local_addr().expect("HTTP test server address should exist");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("CLI should connect");
        let mut request = Vec::new();
        let mut buffer = [0u8; 1024];
        loop {
            let read = stream.read(&mut buffer).expect("request should be readable");
            if read == 0 { break; }
            request.extend_from_slice(&buffer[..read]);
            if request.windows(4).any(|window| window == b"\r\n\r\n") { break; }
        }
        assert!(request.starts_with(b"GET / HTTP/1.1\r\n"));
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 14\r\n\r\nhello from CLI").expect("response should be writable");
    });
    let target = format!("http://{}:{}/", address.ip(), address.port());
    let source = format!("ambil dari \"{target}\"\n");
    let path = fixture(&source);
    let output = run(&["run", path.to_str().unwrap(), "--izinkan", "network.connect", "--target", &target]);
    let _ = fs::remove_file(&path);
    server.join().expect("HTTP server should finish");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello from CLI\n");
}

#[test]
fn cli_http_get_is_denied_without_permission() {
    let path = fixture("ambil dari \"http://127.0.0.1:9/\"\n");
    let output = run(&["run", path.to_str().unwrap()]);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("N3209"));
}

#[test]
fn cli_http_get_is_denied_when_target_is_out_of_scope() {
    let path = fixture("ambil dari \"http://127.0.0.1:9/\"\n");
    let output = run(&["run", path.to_str().unwrap(), "--izinkan", "network.connect", "--target", "http://127.0.0.1:10/"]);
    let _ = fs::remove_file(&path);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("N3209"));
}
