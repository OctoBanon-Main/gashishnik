# Gashishnik
[<img src="https://github.com/user-attachments/assets/f2be5caa-6246-4a6a-9bee-2b53086f9afb" height="30">]()
[<img src="https://github.com/user-attachments/assets/4d35191d-1dbc-4391-a761-6ae7f76ba7af" height="30">]()

**Gashishnik — High-performance RAC protocol server implementation in Rust**

Gashishnik is a server implementation of the Real Address Chat (RAC) protocol, developed by Mr. Sugoma and promoted as the "IRC Killer," designed to manage text-based communication between clients.

> More information about RAC (and related projects such as WRAC) can be found on [MeexReay’s Racinfo](https://racinfo.meex.lol)

## Features
- TLS support (encrypts client-server communication; requires client TLS support)
- Server-side message sanitization (removes ANSI control characters and other unwanted input from clients)
- Password hashing via bcrypt
- SQLite storage with configurable database file
- Authentication-only mode
- RAC and WRACv2.0 protocol support

## Default ports

| Mode  | Insecure (no TLS) | TLS Enabled |
|-------|-----------------|-------------|
| RAC   | 42666           | 42667       |
| WRAC  | 52666           | 52667       |

## Prerequisites

Before building the project, ensure you have the following installed:

[Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## Steps to Build

1. Clone the repository:

First, clone the project repository to your local machine:

```bash
git clone https://github.com/OctoBanon-Main/gashishnik.git
```

Then, navigate to the project directory:

```bash
cd gashishnik
```

2. Build the project:

To compile the project, run the following command:

```bash
cargo build --release
```

The compiled executable will be available in the `/target/release/` directory.

## Usage

Run in default RAC mode:
```bash
./target/release/gashishnik-server -a 0.0.0.0
```

Run with TLS:
```bash
./target/release/gashishnik-server -a 0.0.0.0 --tls-cert server.crt --tls-key server.key
```

Run in WRAC (WebSocket) mode:
```bash
./target/release/gashishnik-server -a 0.0.0.0 --mode wrac
```

Run WRAC with TLS:
```bash
./target/release/gashishnik-server -a 0.0.0.0 --mode wrac --tls-cert server.crt --tls-key server.key
```

Run in authentication-only mode (applies to both RAC and WRAC):
```bash
./target/release/gashishnik-server -a 0.0.0.0 --auth-only
```

**Mode summary:**

- RAC – default TCP-based protocol (one request per connection).
- WRAC – WebSocket-based protocol, allows persistent connections.
- Auth-only – disables unauthenticated message sending in both modes.

## Generating self-signed TLS certificates

**Requirements**
- **OpenSSL** must be installed and available in your PATH.  
  - **Debian/Ubuntu:** `sudo apt install openssl`  
  - **Arch Linux:** `sudo pacman -S openssl`  
  - **macOS:** `brew install openssl`  
  - **Windows:**  
    - Install [Git for Windows](https://gitforwindows.org/) — it includes `openssl.exe`,  
      or  
    - Use [OpenSSL for Windows binaries](https://slproweb.com/products/Win32OpenSSL.html)

**Generate certificate and key**

Run this command in your terminal or PowerShell:

```bash
openssl req -x509 -newkey rsa:4096 -sha256 -days 365 -nodes ^
  -keyout server.key -out server.crt ^
  -subj "/CN=localhost"
```

> 📝 On Linux/macOS, replace ^ with \ for line continuation.

**This creates:**

- **server.crt** — certificate file

- **server.key** — private key file

**Example run with TLS**
```bash
./target/release/gashishnik-server -a 0.0.0.0 \
  --tls-cert server.crt --tls-key server.key
```

> TLS works in both RAC and WRAC modes.

## License

This project is licensed under the [MIT License](https://github.com/OctoBanon-Main/gashishnik/blob/main/LICENSE).

## See also

- [Mefedroniy](https://github.com/OctoBanon-Main/mefedroniy-client) — TUI Rust client for RAC.
- [bRAC](https://github.com/MeexReay/bRAC) — GTK4/Adwaita client on Rust.
- [sRAC](https://github.com/MeexReay/sRAC) — Simple RAC server on Rust.