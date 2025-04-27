use axum::{http::{header, Response, StatusCode}, routing::{get, post}, Json, Router};
use serde_json::json;
use tokio::net::TcpListener;
use jwt_lib::User;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let routes = Router::new()
        .route("/public-view", get(public_view_handler))
        .route("/get-token", post(get_token_handler))
        .route("/secret-view", get(|| async {"secret view"}));

    let tcp_listner = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Address should be free and valid");

    axum::serve(tcp_listner, routes).await.expect("Error serving application");
}

async fn public_view_handler() -> Response<String>{
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(json!(
            {
                "success": true,
                "data":{
                    "message": "This data is available to all users"
                }
            }
        ).to_string()).unwrap_or_default()

}

async fn get_token_handler(Json(user): Json<User>) -> Response<String> {
    let token = jwt_lib::get_jwt(user);
    match token {
        Ok(token) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!(
                {
                    "success": true,
                    "data": {
                        "token": token
                    }
                }
            ).to_string()).unwrap_or_default(),

        Err(err) => Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(json!(
            {
           "status":false,
           "data": {
            "message": String::from("Error Occurred: ")+ &err
           }
        }
        ).to_string()).unwrap_or_default()
    }
}