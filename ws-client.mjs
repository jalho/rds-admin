import WebSocket from 'ws';

const ws = new WebSocket('ws://127.0.0.1:8080');

ws.on('error', console.error);
ws.on("close", () => console.log("closed"));

ws.on('open', function open() {
  ws.send(process.argv[2]);
});

ws.on('message', function message(data) {
  console.log('received: %s', data);
  ws.close(1000);
});
