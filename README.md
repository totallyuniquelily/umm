# Coin catcher or something

This is actually a port of a game I made once when trying to learn/practice Python.
It's not fun or anything, but you can use it as an example of (probably bad) Rust code.
Note: The terminal interaction consists merely of reading and printing lines (no e.g. overwriting).

## The game
The following is largely based on my memory, and I haven't played it in a long time, so there may be inaccuracies.

- You play as the @ character on a short 1d map.
- You can move left or right by typing in r or l and pressing enter.
- You can place down a net "#" by typing in x (and pressing enter).
- Eventually one of the nets may catch a coin "$". You can collect it by walking onto it.

### Made up gameplay example

```
@----
x
@----
r
#@---
xr
##@--
