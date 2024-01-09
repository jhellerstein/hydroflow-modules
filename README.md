Modules for heartbeats and retry/ack.

To illustrate usage, includes a chat server example that imports one or the other module.

## Heartbeat Example
To run the example, open 3 terminals.

In one terminal run the server like so:
```
cargo run -- --demo heartbeat --name "_" --role server --addr 127.0.0.1:12347
```

In another terminal run the first client:
```
cargo run -- --demo heartbeat --name "alice" --role client --server-addr 127.0.0.1:12347
```

In the third terminal run the second client:
```
cargo run -- --demo heartbeat --name "bob" --role client --server-addr 127.0.0.1:12347
```

If you type in the client terminals the messages should appear everywhere.

## Retry/Ack Example
To run the example, open 3 terminals.

In one terminal run the server like so:
```
cargo run -- --demo retry --name "_" --role server --addr 127.0.0.1:12347
```

In another terminal run the first client:
```
cargo run -- --demo retry --name "alice" --role client --server-addr 127.0.0.1:12347
```

In the third terminal run the second client:
```
cargo run -- --demo retry --name "bob" --role client --server-addr 127.0.0.1:12347
```

If you type in the client terminals the messages should appear everywhere.

