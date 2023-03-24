### Rust Chat Application

This is a simple chat application written in Rust using the Rocket web framework. It allows users to send messages to a chat room, and other users can join the room and receive messages in real-time using Server-Sent Events (SSE).

**Requirements**
* Rust
* Cargo

**Usage**
1. Clone this repository to your local machine.
2. Navigate to the project directory in your terminal.
3. Run the command `cargo run` to start the server.
4. Open your web browser and navigate to http://localhost:8000.
5. Enter a chat room name and username, and start chatting!


**Code Overview**

The main components of this application are:

* `Message`: A struct that represents a chat message, with fields for the room name, username, and message text.
* `events()`: An async handler for the "/events" route that sends messages to clients connected to the server via SSE.
* `post()`: A handler for the "/message" route that receives a chat message and broadcasts it to all connected clients.
* `rocket()`: The entry point of the program, which creates a Rocket instance and sets up the application.
Credits





![Screenshot from 2023-03-24 22-34-31](https://user-images.githubusercontent.com/58559626/227597963-9dda3c7c-9b84-4319-8d1a-823102523ffc.png)
