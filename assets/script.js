const url = "ws://localhost:3000/ws?user_id=ashim"
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


