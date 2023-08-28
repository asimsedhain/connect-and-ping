function getRandomInt(max) {
	return Math.floor(Math.random() * max);
}

function getUrl(s) {
	let location = window.location;
	return ((location.protocol === "https:") ? "wss://" : "ws://") + location.host + s;
}

const id = getRandomInt(1000);

const url = getUrl("/ws?user_id=browser-tab-" + id)
const socket = new WebSocket(url);

socket.addEventListener('open', function(_event) {
	socket.send('Hello Server!');
	console.log(`Socket connected to ${url}`);
});

socket.addEventListener("close", function(event) {
	console.log("Socker closed: ", event);
})

socket.addEventListener('message', function(event) {
	console.log('Message from server ', event.data);
});


