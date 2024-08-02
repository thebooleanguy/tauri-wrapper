use std::net::TcpStream;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn start_spring_boot() {
  Command::new("java")
  .arg("-jar")
  .arg("./assets/demo.jar")
  .spawn()
  .expect("Failed to start Spring Boot application");
}

fn wait_for_server(host: &str, port: u16, timeout_seconds: u64) -> bool {
  let start_time = std::time::Instant::now();
  while start_time.elapsed().as_secs() < timeout_seconds {
    match TcpStream::connect((host, port)) {
      Ok(_) => return true,
      Err(_) => thread::sleep(Duration::from_secs(1)),
    }
  }
  false
}

fn main() {
  // Start Spring Boot application in a separate thread
  thread::spawn(|| {
    start_spring_boot();
  });

  // Wait for the Spring Boot server to start
  let host = "127.0.0.1"; // Adjust as needed
  let port = 8080;        // Adjust as needed
  let server_ready = wait_for_server(host, port, 30); // Wait up to 30 seconds

  if !server_ready {
    eprintln!("Failed to connect to the Spring Boot server.");
    return;
  }

  // Proceed with Tauri setup
  tauri::Builder::default()
  .setup(|_app| {
    Ok(())
  })
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
