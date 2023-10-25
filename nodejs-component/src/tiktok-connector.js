const axios = require('axios');

const BASE_URL = 'http://127.0.0.1:3000';

async function sendRequest(endpoint) {
    try {
        const response = await axios.post(`${BASE_URL}/${endpoint}`, {
            data: 'sample payload'
        });
        console.log(`Response from ${endpoint}:`, response.status, response.data);
    } catch (error) {
        if (error.response) {
            console.error(`Error sending ${endpoint} request:`, error.response.status, error.response.data);
        } else if (error.request) {
            // The request was made but no response was received
            console.error(`No response received when sending ${endpoint} request.`, error.message);
        } else {
            // Something happened in setting up the request that triggered an Error
            console.error('Error setting up the request:', error.message);
        }
    }
}

let totalLikes = 0;  // To track total likes
let lastPrintedAt = 0;  // To track the last count where "Hello World" was printed

const { WebcastPushConnection } = require('tiktok-live-connector');

// Username of someone who is currently live
let tiktokUsername = "mrcoffeetv";

// Create a new wrapper object and pass the username
let tiktokLiveConnection = new WebcastPushConnection(tiktokUsername);

// Connect to the chat (await can be used as well)
tiktokLiveConnection.connect().then(state => {
    console.info(`Connected to roomId ${state.roomId}`);
}).catch(err => {
    console.error('Failed to connect', err);
})

tiktokLiveConnection.on('like', data => {
    console.log(`${data.nickname} sent ${data.likeCount} likes`);
    totalLikes = data.totalLikeCount

    if (Math.floor(totalLikes / 10) > lastPrintedAt) {
        lastPrintedAt = Math.floor(totalLikes / 10);
        sendRequest("like");
    }    
})

tiktokLiveConnection.on('share', (data) => {
    console.log(data.nickname, "shared the stream!");
    sendRequest("share");
})

tiktokLiveConnection.on('follow', (data) => {
    console.log(data.nickname, "followed!");
    sendRequest("follow")
})

tiktokLiveConnection.on('gift', data => {
    console.log(`${data.nickname} sent ${data.giftName}`);
    switch(data.giftName) {
        case "Star":
            sendRequest("reset");
            return;
        case "Galaxy":
            println("todo");
            return;
    }
    sendRequest("gift");
})
