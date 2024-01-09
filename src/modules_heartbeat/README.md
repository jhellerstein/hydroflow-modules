A module for tracking members, broadcasting heartbeats, collecting responses, and culling members.

To illustrate usage, includes a chat server example that imports the module.

To run the example, open 3 terminals.

In one terminal run the server like so:
```
cargo run -p hydroflow --example modules_heartbeat -- --name "_" --role server --addr 127.0.0.1:12347
```

In another terminal run the first client:
```
cargo run -p hydroflow --example modules_heartbeat -- --name "alice" --role client --server-addr 127.0.0.1:12347
```

In the third terminal run the second client:
```
cargo run -p hydroflow --example modules_heartbeat -- --name "bob" --role client --server-addr 127.0.0.1:12347
```

If you type in the client terminals the messages should appear everywhere.

Adding the `--graph <graph_type>` flag to the end of the command lines above will print out a node-and-edge diagram of the program. Supported values for `<graph_type>` include [mermaid](https://mermaid-js.github.io/) and [dot](https://graphviz.org/doc/info/lang.html).
