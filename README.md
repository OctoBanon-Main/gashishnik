# Gashishnik
[<img src="https://github.com/user-attachments/assets/f2be5caa-6246-4a6a-9bee-2b53086f9afb" height="30">]()
[<img src="https://github.com/user-attachments/assets/4d35191d-1dbc-4391-a761-6ae7f76ba7af" height="30">]()

**Gashishnik — Server implementation for Real Address Chat (RAC).**

Gashishnik is a server implementation of the Real Address Chat (RAC) protocol, developed by Mr. Sugoma and promoted as the "IRC Killer," designed to manage text-based communication between clients.

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

And run with this command:

```bash
./target/release/gashishnik 127.0.0.1 6667
```

Optionally, you can specify TLS certificate and key files using the --cert and --key flags:

```bash
./target/release/gashishnik 127.0.0.1 6667 --cert path/to/cert.pem --key path/to/key.pem
```

The compiled executable will be available in the `/target/release/` directory.

Alternatively, you can download the precompiled binary from the [Releases](https://github.com/OctoBanon-Main/gashishnik/releases) page.

## License

This project is licensed under the [MIT License](https://github.com/OctoBanon-Main/gashishnik/blob/main/LICENSE).

## See also

- [RAC-hub](https://forbirdden.github.io/RAC-Hub) - List of all clients and servers for RAC
- [Mefedroniy](https://github.com/OctoBanon-Main/mefedroniy-client) — TUI Rust client for RAC.
- [RAC protocol v2.0](https://gitea.bedohswe.eu.org/pixtaded/crab#rac-protocol) — The Real Address Chat protocol.
- [RAC protocol v1](https://bedohswe.eu.org/text/rac/protocol.md.html) - First version of the protocol.
- [bRAC](https://github.com/MeexReay/bRAC) - GTK4 Rust client for RAC.
- [CRAB](https://gitea.bedohswe.eu.org/pixtaded/crab) - Client and server for RAC on Java.
- [AlmatyD](https://gitea.bedohswe.eu.org/bedohswe/almatyd) - Unofficial server implementation for RAC.
- Lobster - QT6 C++ client for RAC.
