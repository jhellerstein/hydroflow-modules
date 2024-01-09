use std::time::Duration;

use hydroflow::hydroflow_syntax;
use hydroflow::scheduled::graph::Hydroflow;
use hydroflow::util::{UdpSink, UdpStream};

use crate::modules_retry_ack::chat::protocol::{Message, MessageWithAddr, RetryMsg};
use crate::Opts;

pub(crate) async fn run_server(outbound: UdpSink, inbound: UdpStream, _opts: Opts) {
    println!("Server live!");

    let mut hf: Hydroflow = hydroflow_syntax! {
        retry_send = import!("../sender.hf");

        // Define shared inbound and outbound channels
        outbound_chan = union()
            -> [input]retry_send
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

        // Pipeline 3: Handle Acks of messages
        inbound_chan[RetryAck]
            // -> map(|(addr, msg_id, )| (Message::RetryAck{msg_id}, addr))
            -> map(|(addr, msg)| (msg, addr))
            -> [ack]retry_send;
    };

    hf.run_async().await.unwrap();
}
