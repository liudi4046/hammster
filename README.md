# ZK Hammster2

This app is inspired by this great tutorial : https://medium.com/@yujiangtham/building-a-zero-knowledge-web-app-with-halo-2-and-wasm-part-1-80858c8d16ee

## Why I did this

In the ZK Hammster, we are asked to enter two numbers to generate proof. then the proof is stored in localstorage in browser. In verification page, the proof is retreived to verify.

This makes me confused initially. If I don't understand the code and the zk behind it, I would not be able to figure out what this app is doing. So I adjusted the whole process to make it more make sense.

## How the app works

### Proof generation

```
cd hammster2
cargo run
```

This would pop up a GUI. Enter two of 8 binary(0 or 1),and enter their hamming distance.Then click Prove button.

A proof file will be generated at the current dir.

### Verification

```
cd frontend
npm start
```

upload the proof file you just generated. And enter the hamming distance. then verify.

If the proof and the hamming distance is matched(means correct),then it will accept, otherwise reject.

So basically, you can prove you know two 8-bit binary numbers, so that their hamming distance is the num you claimed. And you don't need to reveal the two numbers.
