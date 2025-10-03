# Gashishnik
[<img src="https://github.com/user-attachments/assets/f2be5caa-6246-4a6a-9bee-2b53086f9afb" height="30">]()
[<img src="https://github.com/user-attachments/assets/4d35191d-1dbc-4391-a761-6ae7f76ba7af" height="30">]()

**Gashishnik — High-performance RAC protocol server implementation in Rust**

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
./target/release/gashishnik --bind-ip 127.0.0.1 --bind-port 42666
```

The compiled executable will be available in the `/target/release/` directory.

Alternatively, you can download the precompiled binary from the [Releases](https://github.com/OctoBanon-Main/gashishnik/releases) page.

## License

This project is licensed under the [MIT License](https://github.com/OctoBanon-Main/gashishnik/blob/main/LICENSE).

## See also

- [Mefedroniy](https://github.com/OctoBanon-Main/mefedroniy-client) — TUI Rust client for RAC.
- [sRAC](https://github.com/MeexReay/sRAC) — Simple RAC server on Rust.