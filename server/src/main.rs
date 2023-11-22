mod game;
mod network;

fn main() {
    let mut server = network::Server::new();
    let _ = server.launch_server();
}
