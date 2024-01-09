use std::net::SocketAddr;
use std::time::Duration;

use chrono::{DateTime, Utc};
use hydroflow::hydroflow_syntax;
use hydroflow::scheduled::graph::Hydroflow;
use hydroflow::util::{UdpSink, UdpStream};

use crate::modules_heartbeat::chat::protocol::{Message, MessageWithAddr};
use crate::Opts;

pub(crate) async fn run_server(outbound: UdpSink, inbound: UdpStream, _opts: Opts) {
    println!("Server live!");

    let mut hf: Hydroflow = hydroflow_syntax! {
        // Define shared inbound and outbound channels
        outbound_chan = union()
            // -> inspect(|(m, a)| println!("Sending {:?} to {:?}", m, a))
            -> dest_sink_serde(outbound);
        inbound_chan = source_stream_serde(inbound)
            -> map(Result::unwrap)
            -> map(|(msg, addr)| MessageWithAddr::from_message(msg, addr))
            -> demux_enum::<MessageWithAddr>();
        clients = inbound_chan[ConnectRequest] -> map(|(addr,)| addr) -> tee();
        inbound_chan[ConnectResponse] -> for_each(|(addr,)| println!("Received unexpected `ConnectResponse` as server from addr {}.", addr));

        // Pipeline 1: Acknowledge client connections
        clients[0] -> map(|addr| (Message::ConnectResponse, addr)) -> [0]outbound_chan;

        // Pipeline 2: Broadcast messages to all clients
        inbound_chan[ChatMsg] -> map(|(_addr, nickname, message, ts)| Message::ChatMsg { nickname, message, ts }) -> [0]broadcast;
        clients[1] -> [1]broadcast;
        broadcast = cross_join::<'tick, 'static>() -> [1]outbound_chan;

        // setup heartbeats with clients
        heartbeat = import!("../heartbeat.hf");
        clients -> [members]heartbeat;
        inbound_chan[HeartbeatResponse] -> [response]heartbeat;
        inbound_chan[HeartbeatMsg] -> [msg]heartbeat;
        inbound_chan[HeartbeatDisconnect] -> [disconnect]heartbeat;
        heartbeat -> outbound_chan;
    };

    hf.run_async().await.unwrap();
}
