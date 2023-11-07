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
let lastPrintedAt = 0; // To track the last count where "Hello World" was printed

const { WebcastPushConnection } = require("tiktok-live-connector");

const WebSocket = require("ws");

// Username of someone who is currently live
let tiktokUsername = "crocvip";

let ws = new WebSocket("ws://127.0.0.1:4430/vtplus");

// Create a new wrapper object and pass the username
let tiktokLiveConnection = new WebcastPushConnection(tiktokUsername);

ws.on("open", function open() {
  console.log("Connected to WebSocket server");
});

ws.on("message", function incoming(data) {
  console.log(data);
});

// Timer related variables and functions for Follows
var followTimerId = null;
var followTimeLeft = 0;

function addFollowTime() {
  followTimeLeft += 5;
  if (followTimerId === null) {
    let message = `VTP_HeadPat:50`;
    ws.send(message);
    startFollowTimer();
  }
}

function startFollowTimer() {
  if (followTimerId !== null) {
    clearTimeout(followTimerId);
  }
  followTimerId = setTimeout(function tick() {
    if (followTimeLeft > 0) {
      followTimeLeft -= 1;
      console.log("Follow Time left: " + followTimeLeft + " seconds");
      followTimerId = setTimeout(tick, 1000);
    } else {
      clearTimeout(followTimerId);
      followTimerId = null;
      let message = `VTP_HeadPat:50`;
      ws.send(message);
    }
  }, 1000);
}

// Timer related variables and functions for Shares
var shareTimerId = null;
var shareTimeLeft = 0;

function addShareTime() {
  shareTimeLeft += 5;
  if (shareTimerId === null) {
    let message = `VTP_FX:Rainbow`;
    ws.send(message);
    startShareTimer();
  }
}

function startShareTimer() {
  if (shareTimerId !== null) {
    clearTimeout(shareTimerId);
  }
  shareTimerId = setTimeout(function tick() {
    if (shareTimeLeft > 0) {
      shareTimeLeft -= 1;
      console.log("Share Time left: " + shareTimeLeft + " seconds");
      shareTimerId = setTimeout(tick, 1000);
    } else {
      clearTimeout(shareTimerId);
      shareTimerId = null;
      let message = `VTP_FX:Rainbow`;
      ws.send(message);
    }
  }, 1000);
}

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
  totalLikes += data.likeCount;

  sendRequest("like", data.likeCount);
  let Count = data.likeCount;
  let ItemIndex = 8;
  let CustomItemIndex = -1;
  let Damage = 0;

  let message = `VTP_Throw:${Count}:${ItemIndex}:${CustomItemIndex}:${Damage}`;

  ws.send(message);
});

tiktokLiveConnection.on("share", (data) => {
  console.log(data.nickname, "shared the stream!");
  addShareTime();
  sendRequest("share", 0);
});

tiktokLiveConnection.on("follow", (data) => {
  console.log(data.nickname, "followed!");
  addFollowTime();
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
