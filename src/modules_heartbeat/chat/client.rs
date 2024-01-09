use chrono::prelude::*;
use colored::Colorize;
use hydroflow::hydroflow_syntax;
use hydroflow::util::{UdpSink, UdpStream};

use crate::modules_heartbeat::chat::protocol::{Message, MessageWithAddr};
use crate::Opts;

fn pretty_print_msg(nickname: String, message: String, ts: DateTime<Utc>) {
    println!(
        "{} {}: {}",
        ts.with_timezone(&Local)
            .format("%b %-d, %-I:%M:%S")
            .to_string()
            .truecolor(126, 126, 126)
            .italic(),
        nickname.green().italic(),
        message,
    );
}

pub(crate) async fn run_client(outbound: UdpSink, inbound: UdpStream, opts: Opts) {
    // server_addr is required for client
    let server_addr = opts.server_addr.expect("Client requires a server address");
    println!("Client live!");

    let mut hf = hydroflow_syntax! {
        // set up channels
        outbound_chan = union() -> dest_sink_serde(outbound);
        // inbound_chan = source_stream_serde(inbound)
        //     -> map(Result::unwrap)
        //     -> map(|(msg, _addr)| msg)
        //     -> demux_enum::<Message>();
        inbound_chan = source_stream_serde(inbound)
        -> map(Result::unwrap)
        -> map(|(msg, addr)| MessageWithAddr::from_message(msg, addr))
        -> demux_enum::<MessageWithAddr>();
        inbound_chan[ConnectRequest] -> for_each(|_| println!("Received unexpected connect request from server."));

        // send a single connection request on startup
        initialize() -> map(|_m| (Message::ConnectRequest, server_addr)) -> [0]outbound_chan;

        // take stdin and send to server as a msg
        // the batch serves to buffer msgs until the connection request is acked
        lines = source_stdin()
          -> map(|l| Message::ChatMsg {
                    nickname: opts.name.clone(),
                    message: l.unwrap(),
                    ts: Utc::now()})
          -> [input]msg_send;
        inbound_chan[ConnectResponse] -> persist() -> [signal]msg_send;
        msg_send = defer_signal() -> map(|msg| (msg, server_addr)) -> [1]outbound_chan;

        // receive and print messages
        inbound_chan[ChatMsg] -> for_each(|(_addr, nick, msg, ts)| pretty_print_msg(nick, msg, ts));

        hb_response = import!("../heartbeat_response.hf");
        inbound_chan[HeartbeatMsg] -> [msg]hb_response;
        inbound_chan[HeartbeatResponse] -> [response]hb_response;
        inbound_chan[HeartbeatDisconnect] -> [disconnect]hb_response;
        hb_response -> outbound_chan;

    };

    hf.run_async().await.unwrap();
}
