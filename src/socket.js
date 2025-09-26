const url = 'ws://localhost:8080/ws'; // Replace with your WebSocket URL
export let connection = new WebSocket(url);

console.log("Hello!");

connection.onopen = () => {
    console.log('WebSocket is open now.');
    connection.send(JSON.stringify({message: 'Hello Server!'})); // Example message to send
};

connection.onmessage = (event) => {
    console.log(`Message from server: ${event.data}`);
};

connection.onerror = (error) => {
    console.error(`WebSocket error: ${error}`);
};

connection.onclose = () => {
    console.log('WebSocket is closed now.');
};