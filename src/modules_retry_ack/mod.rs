use hydroflow::util::{bind_udp_bytes, ipv4_resolve};

use crate::modules_retry_ack::chat::client::run_client;
use crate::modules_retry_ack::chat::server::run_server;

mod chat;

// #[hydroflow::main]
pub async fn retry_ack_main(opts: crate::Opts) {
    // if no addr was provided, we ask the OS to assign a local port by passing in "localhost:0"
    let addr = opts
        .addr
        .unwrap_or_else(|| ipv4_resolve("localhost:0").unwrap());

    // allocate `outbound` sink and `inbound` stream
    let (outbound, inbound, addr) = bind_udp_bytes(addr).await;
    println!("Listening on {:?}", addr);

    match opts.role {
        crate::Role::Client => {
            run_client(outbound, inbound, opts).await;
        }
        crate::Role::Server => {
            run_server(outbound, inbound, opts).await;
        }
    }
}

#[test]
fn test() {
    use std::io::Write;

    use hydroflow::util::{run_cargo_example, wait_for_process_output};

    let (_server, _, mut server_output) =
        run_cargo_example("chat", "--role server --name server --addr 127.0.0.1:11247");

    let mut server_output_so_far = String::new();
    wait_for_process_output(
        &mut server_output_so_far,
        &mut server_output,
        "Server live!",
    );

    let (_client1, mut client1_input, mut client1_output) = run_cargo_example(
        "chat",
        "--role client --name client1 --server-addr 127.0.0.1:11247",
    );

    let (_client2, _, mut client2_output) = run_cargo_example(
        "chat",
        "--role client --name client2 --server-addr 127.0.0.1:11247",
    );

    let mut client1_output_so_far = String::new();
    let mut client2_output_so_far = String::new();

    wait_for_process_output(
        &mut client1_output_so_far,
        &mut client1_output,
        "Client live!",
    );
    wait_for_process_output(
        &mut client2_output_so_far,
        &mut client2_output,
        "Client live!",
    );

    client1_input.write_all(b"Hello\n").unwrap();

    wait_for_process_output(
        &mut client2_output_so_far,
        &mut client2_output,
        ".*, .* client1: Hello",
    );
}
