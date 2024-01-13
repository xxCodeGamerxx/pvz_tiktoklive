## Plants Vs Zombies + TikTok Live Connector

A small modding project for setting up Plants Vs Zombies to react to TikTok Live's gifting/like/follower events. I was trying to get spawning zombies to work, but the only thing I ended up getting to work before giving up was change the sun count. 

THIS PROJECT ONLY WORKS WITH PVZ VERSION 1.0.0.1052

## How to set up

Clone this project, then:

- Install [Nodejs](https://nodejs.org/en)
- Install [Rust](https://www.rust-lang.org/learn/get-started)

Go to your ./rust_component directory, and while PVZ is running, type

```
cargo run
```

This runs a local server at http://127.0.0.1:3000 .

In terminal in ./node-component directory, use
```
npm i tiktok-live-connector
```

Go src/tiktok-connector.js and change this line of code

```
let tiktokUsername = "crocvip";
```

to someone who is currently live. 

Then start tiktok-connector server with
```
node src/tiktok-connector.js
```

Note: It is recommended you use a VPN while using the tiktok connector, as tiktok gets big mad if you do this for too long and you might get IP banned.
