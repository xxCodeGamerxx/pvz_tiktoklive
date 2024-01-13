const axios = require("axios");

const BASE_URL = "http://127.0.0.1:3000";

async function sendRequest(endpoint, likeCount) {
  try {
    const url =
      likeCount !== undefined
        ? `${BASE_URL}/${endpoint}/${likeCount}`
        : `${BASE_URL}/${endpoint}`;
    const response = await axios.post(url, {
      data: "sample payload",
    });
  } catch (error) {
    if (error.response) {
      console.error(
        `Error sending ${endpoint} request:`,
        error.response.status,
        error.response.data
      );
    } else if (error.request) {
      // The request was made but no response was received
      console.error(
        `No response received when sending ${endpoint} request.`,
        error.message
      );
    } else {
      // Something happened in setting up the request that triggered an Error
      console.error("Error setting up the request:", error.message);
    }
  }
}

let totalLikes = 0; // To track total likes

const { WebcastPushConnection } = require("tiktok-live-connector");

// Username of someone who is currently live
let tiktokUsername = "crocvip";

// Create a new wrapper object and pass the username
let tiktokLiveConnection = new WebcastPushConnection(tiktokUsername);


// Connect to the chat (await can be used as well)
tiktokLiveConnection
  .connect()
  .then((state) => {
    console.info(`Connected to roomId ${state.roomId}`);
  })
  .catch((err) => {
    console.error("Failed to connect", err);
  });

tiktokLiveConnection.on("like", (data) => {
  console.log(`${data.nickname} sent ${data.likeCount} likes`);

  sendRequest("like", data.likeCount);
});

tiktokLiveConnection.on("share", (data) => {
  console.log(data.nickname, "shared the stream!");
  sendRequest("share", 0);
});

tiktokLiveConnection.on("follow", (data) => {
  console.log(data.nickname, "followed!");
  sendRequest("follow", 1);
});

tiktokLiveConnection.on("gift", (data) => {
  console.log(`${data.nickname} sent ${data.giftName}`);
  switch (data.giftName) {
    case "Little Crown":
      sendRequest("reset", 1);
      return;
    case "Paper Crane":
      sendRequest("reset", 1);
      return;
  }
  sendRequest("gift", data.repeatCount);
});

tiktokLiveConnection.on("disconnected", () => {
  console.log("disconnected :(");
});

tiktokLiveConnection.on("streamEnd", (actionId) => {
  if (actionId === 3) {
    console.log("Stream ended by user!");
  }
  if (actionId === 4) {
    console.log("User was banned!");
  }
});
