# Tauri Wrapper

This repository contains a Tauri wrapper for personal use. The project is public for anyone who might find it useful.

## Features
- Cross-platform desktop application
- Lightweight and fast

## Installation

Clone the repository:
```sh
git clone https://github.com/yourusername/tauri-wrapper.git
cd tauri-wrapper
```

### Requirements

- Rust (Install from [rustup.rs](https://rustup.rs/) or your package manager)
- Node.js (Install from [nodejs.org](https://nodejs.org/) or your package manager)
- Tauri CLI (Install with `cargo install tauri-cli`)

### Setup

1. **Add the JAR package to the assets directory:**
   Place your `.jar` package in the `assets` directory. Ensure all paths are correctly set as class paths within the JAR.

2. **Edit `tauri.conf.json`:**
   Update the `tauri.conf.json` file to match your project's paths and names. Here’s an example configuration:
   ```json
{
  "build": {
    "beforeBuildCommand": "",
    "beforeDevCommand": "",
    "devPath": ".",
    "distDir": "./assets"
  },
  "package": {
    "productName": "Dictionary WebApp",
    "version": "1.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false
    },
    "bundle": {
      "active": true,
      "category": "Education",
      "deb": {
        "depends": []
      },
      "externalBin": [],
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ],
      "identifier": "com.thebooleanguy.dictionary",
      "longDescription": "A dictionary web application for educational purposes.",
      "macOS": {
        "entitlements": null,
        "exceptionDomain": "",
        "frameworks": [],
        "providerShortName": null,
        "signingIdentity": null
      },
      "resources": ["./assets/dictionary-app-web.jar"],
      "shortDescription": "Dictionary WebApp",
      "targets": ["appimage"],
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "http://timestamp.digicert.com"
      }
    },
    "security": {
      "csp": null
    },
    "updater": {
      "active": false
    },
    "windows": [
      {
        "url": "http://localhost:8080",
        "fullscreen": false,
        "height": 600,
        "resizable": true,
        "title": "Dictionary",
        "width": 800
      }
    ]
  }
}

   ```

3. **Check `src/main.rs`:**
   Ensure that the `src/main.rs` file in your Rust project is configured correctly. Here’s an example:
   ```rust
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
   ```

### Building the Application


### Running the Application

To run the application in development mode:
```sh
cargo tauri dev
```

### Building the Application for Distribution

To build the application for distribution:
```sh
cargo tauri build
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
