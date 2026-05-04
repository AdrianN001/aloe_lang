---

# Async TCP Client/Server (Aloe)

A minimal example of an asynchronous TCP client and server built with Aloe, demonstrating non-blocking I/O and concurrent connection handling.

---

## 📦 Overview

This project consists of two components:

* `tcp_server.aloe` — A TCP server that accepts incoming connections and prints received messages
* `tcp_client.aloe` — A TCP client that sends user input to the server

Communication happens over a local TCP connection (`127.0.0.1:4560`).

---

## 🚀 Features

* Asynchronous networking (non-blocking I/O)
* Concurrent client handling using task spawning (`__spawn`)
* Simple message logging on the server
* Lightweight byte-to-string conversion

---

## 🖥️ Server

The server:

* Binds to `127.0.0.1:4560`
* Accepts incoming client connections
* Spawns a new task per connection
* Reads incoming data and prints it to stdout

### Core Function

```aloe
async fun handle_connection(client_stream)
```

* Handles a single client connection
* Continuously reads data from the stream
* Terminates when the client disconnects

---

## 💻 Client

The client:

* Connects to the server
* Reads user input from stdin
* Sends input as raw bytes

### Main Loop

```aloe
while (inpt = __input()) {
    await server_stream.write(inpt.as_byte_array()); 
}
```

---

## 🔌 Communication Details

* Protocol: TCP
* Encoding: Raw bytes (interpreted as ASCII on the server)
* No framing or message boundaries beyond stream behavior

---

## ▶️ Usage

### 1. Start the Server

```bash
cargo run tcp_server.aloe
```

Expected output:

```
Server runs on 127.0.0.1 : 4560
```

---

### 2. Start the Client

```bash
cargo run tcp_client.aloe
```

Expected output:

```
Connected to the server on [127.0.0.1:4560]
```

---

### 3. Send Messages

* Type into the client terminal
* Messages will appear on the server

---

## 🧠 Architecture

* Each client connection runs in its own async task (`__spawn`)
* Fully asynchronous I/O via `await`
* No explicit synchronization primitives required

