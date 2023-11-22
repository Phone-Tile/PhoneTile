mod network;
mod game;

fn main() {
    let mut server = network::Server::new();
    let _ = server.launch_server();
}
