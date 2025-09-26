use axum::{
    Json, Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::Response,
    routing::{any, get, post},
};
use tower_http::services::ServeDir;

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let static_files_service = ServeDir::new("../dist/static");
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/ws", any(handler))
        .nest_service("/static", static_files_service)
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag="type")]
enum ClientMessage {
    Bet { amount: f32 },
	Hit { token: String },
	Stand { token: String }
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    println!("ATTEMPTING WEBSOCKET CONNECTION");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
			use ClientMessage::*;
            match &msg {
                Message::Text(utf8_bytes) => {
                    match serde_json::from_str::<ClientMessage>(utf8_bytes.as_str()) {
                        Ok(msg) => match msg {
                            Bet { amount } => {
								println!("apuesta {}€", amount);
							},
							Hit => {},
							Stand => {}
                        },
                        Err(err) => println!("{}", err),
                    }
                }
                _ => println!("invalid message"),
            };

            if socket.send(msg).await.is_err() {
                // client disconnected
                return;
            }
        } else {
            // client disconnected
            return;
        };
    }
}
