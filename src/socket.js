export const socket = new WebSocket("ws://localhost:8080/ws");
socket.onopen = () => {
  console.log("Connected to the server");
  socket.send("Hello from the client side :3");
};

export let secreto = "TEMP";

export let dinero_cuenta = "1000";