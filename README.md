# maelstrom-challenges

## Building & running

Build each task separately and run the binary with the maelstrom utility:

```
# In the project root
cargo build --bin echo

# In the maelstrom test root
./maelstrom test -w echo --bin ../maelstrom-challenges/target/debug/echo --time-limit 3
```
