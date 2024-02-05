# jojo-discovery

True to its name, this repository encompasses the discovery functionality of [jojo](https://github.com/gggiulio77/jojo). It operates on the host computer, alongside [jojo-server](https://github.com/gggiulio77/jojo-server). Its primary function is to listen for "hello" messages from [jojo-client](https://github.com/gggiulio77/jojo-client) and respond with the local address of the host. While it can function independently, the primary intention is to run it alongside the [jojo-app](https://github.com/gggiulio77/jojo-app) as a separate process.

## Getting Started

Essentially, it initiates a UDP socket on the host computer and listens for a broadcast message at `BROADCAST_BIND_ADDRESS`. Subsequently, [jojo-client](https://github.com/gggiulio77/jojo-client) sends a "hello" message via broadcast. The server then responds with its IP address and port. With this information, [jojo-client](https://github.com/gggiulio77/jojo-client) can establish a connection to the WebSocket server ,[jojo-server](https://github.com/gggiulio77/jojo-server), running on the host computer.

### Quick Links

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [License](#license)

### Prerequisites

Before proceeding, ensure you have Rust installed on your system.

### Installation

`git clone https://github.com/gggiulio77/jojo-server.git`

## Usage

To execute the project as a binary, utilize the `cargo run` command. The main.rs file serves as the entry point. Within this file, parameters are hardcoded to invoke the `initialize` function located inside lib.rs. This setup mirrors how the [jojo-app](https://github.com/gggiulio77/jojo-app) will initialize this server in a different process.

You must provide a `BROADCAST_BIND_ADDRESS` as shown in `.env.example` file.

## Roadmap

- [ ] Implement some sort of authentication
- [ ] Implement some sort of encryption
- [ ] Try a mDNS approach
- [ ] Try a multicast approach

## License
