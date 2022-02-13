# Example STUN socket P2P connection

```bash
# First terminal
cargo run --example server

# Second terminal - client #1
# Client "first" connecting to client "second"
# first -> coordination_server <- second
# first -> second
cargo run --example client first second

# Third terminal - client #2
# Client "second" connecting to client "first"
# second -> coordination_server <- first
# second -> first
cargo run --example client second first
```
