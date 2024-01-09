use std::net::SocketAddr;

use clap::{Parser, ValueEnum};
use hydroflow::tokio;
use hydroflow::util::ipv4_resolve;

mod modules_heartbeat;
mod modules_retry_ack;

#[derive(Clone, ValueEnum, Debug)]
enum Role {
    Client,
    Server,
}

#[derive(Clone, ValueEnum, Debug)]
enum Demo {
    Heartbeat,
    Retry,
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    name: String,
    #[clap(value_enum, long)]
    role: Role,
    #[clap(value_enum, long)]
    demo: Demo,
    #[clap(long, value_parser = ipv4_resolve)]
    addr: Option<SocketAddr>,
    #[clap(long, value_parser = ipv4_resolve)]
    server_addr: Option<SocketAddr>,
}

#[hydroflow::main]
async fn main() {
    let opts = Opts::parse();
    // if no addr was provided, we ask the OS to assign a local port by passing in "localhost:0"
    match opts.demo {
        crate::Demo::Heartbeat => {
            crate::modules_heartbeat::heartbeat_main(opts).await;
        }
        crate::Demo::Retry => {
            crate::modules_retry_ack::retry_ack_main(opts).await;
        }
    }
}
