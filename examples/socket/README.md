# Simple TCP Client & Server (Aloe)

This project contains two small programs written in **Aloe** that demonstrate a basic TCP client-server interaction.

* `tcp_server.aloe` → TCP server that receives messages and responds in uppercase
* `tcp_client.aloe` → TCP client that sends user input to the server

---

## 📡 Overview

The server listens on a local TCP socket and processes incoming messages.
The client connects to the server, sends user input, and prints the response.

**Behavior:**

* The client sends a message
* The server converts it to uppercase (except spaces)
* The server sends the result back
* The client prints the response

---

## 🧠 How It Works

### Server (`tcp_server.aloe`)

* Binds to `127.0.0.1:6557`
* Waits for a client connection
* Reads incoming messages
* Converts them to uppercase
* Sends the result back to the client
* Stops when receiving an empty/newline message

### Client (`tcp_client.aloe`)

* Connects to `127.0.0.1:6557`
* Reads input from the user
* Sends it to the server
* Waits for a response
* Prints the server response

---

## 🚀 Usage

### 1. Start the server

```bash
cargo run examples/socket/tcp_server.aloe
```

### 2. Start the client (in another terminal)

```bash
cargo run examples/socket/tcp_client.aloe
```

### 3. Send messages

Type messages into the client.
You should see responses like:

```
hello world
from the server: HELLO WORLD
```

---

## 🧪 Example

**Client input:**

```
hello aloe
```

**Server output:**

```
raw message: hello aloe     message received converted to uppercase => HELLO ALOE
```

**Client output:**

```
from the server: HELLO ALOE
```

