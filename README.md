# maelstrom-challenges

## Building & running

Build each task separately and run the binary with the maelstrom utility:

```
# In the project root
cargo build

# In the maelstrom test root
./maelstrom test -w echo --bin ../maelstrom-challenges/target/debug/maelstrom-challenges --time-limit 3
```
