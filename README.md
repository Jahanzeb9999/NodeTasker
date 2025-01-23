# Node Tasker

Node Tasker is a distributed system framework designed to manage tasks across a network of nodes. It allows efficient task distribution and coordination, providing a robust, scalable solution for executing tasks concurrently in a multi-node environment.

## Features

- **Distributed Task Management**: Effortlessly distribute tasks across nodes in a network.
- **Concurrency**: Leverage asynchronous programming to ensure high throughput and minimal blocking.
- **Task Broadcasting**: Utilize a broadcast channel to send tasks to all nodes concurrently.
- **Thread-Safe Access**: Safely manage shared state with `RwLock` for read and write operations.
- **Unique Node Identification**: Every node in the network is identified by a unique `UUID`.
- **Flexible Integration**: Easily integrate with other systems by adding custom task types.

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Tokio runtime (for asynchronous operations)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/node-tasker.git
   cd node-tasker
   ```

2. Build
   ```bash
   cargo build
   ```
3 Run
   ```bash
   cargo run
   ```
