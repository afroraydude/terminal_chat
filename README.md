# Yuttari: Open-Source Chat

Yuttari is a free and open-source lightweight chat application with secure
end-to-end encryption between users and the server. 

## Features
- Secure end-to-end encryption between users and the server
- Lightweight and fast
- No ads
- No tracking
- No analytics
- No data collection

## Installation

### Client
#### Windows
Download the latest release from the 
[releases page](https://github.com/afroraydude/yuttari-client/releases) 
and run the installer.

#### Linux
Download the latest source code from the 
[releases page](https://github.com/afroraydude/yuttari-client/releases) 
and extract it. Then run the following commands:
```bash
cd yuttari-client
cargo build --release
```
The binary will be located in `target/release/yuttari`.

#### macOS
Download the latest .app from the 
[releases page](https://github.com/afroraydude/yuttari-client/releases) and 
run it.

### Server
#### Windows, Linux, and macOS
Download the latest source code from the
[releases page](https://github.com/afroraydude/yuttari-server/releases)
and extract it. Then run the following commands:
```bash
cd yuttari-server
cargo build --release
```
The binary will be located in `target/release/yuttarid`.

## Usage
### Creating an account
To create an account, simply run the application. On the initial screen,
the application will ask for a username. This username will be used to
identify you on the server. After entering a username, click the "Done"
button. The application will then setup the encryption keys and connect
to the server. Once the connection is established, you will be able to
send and receive messages.

### Sending a message
To send a message, simply type a message in the text box at the bottom
of the screen and press enter. The message will be sent to the server
and then to all other users.
