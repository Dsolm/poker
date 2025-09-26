import { mount } from "svelte";
import App from "./App.svelte";
import './socket.js'
import "./index.css";

const app = mount(App, {
	target: document.body,
	props: {
		name: "world",
	},
});

export default app;
