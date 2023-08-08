function getRandomInt(max) {
  return Math.floor(Math.random() * max);
}
const random = getRandomInt(1000);

const url = "ws://localhost:3000/ws?user_id=browser-tab-"+random
const socket = new WebSocket(url);

socket.addEventListener('open', function(_event) {
	socket.send('Hello Server!');
	console.log(`Socket connected to ${url}`);
});

socket.addEventListener("close", function(_ev) {
	console.log("Socker closed");
})

socket.addEventListener('message', function(event) {
	console.log('Message from server ', event.data);
});


