//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-cors
//! ```

use axum::{http::{HeaderValue, Method}, response::{Html, IntoResponse}, routing::get, routing::post, routing::options, Json, Router,  http};
use std::net::SocketAddr;
use axum::http::{header, HeaderMap, StatusCode};
use tower_http::cors::CorsLayer;
use tower_http::{
    services::{ServeDir},
};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let frontend = async {
        let app = Router::new().route("/", get(index))
            .nest_service("/static", ServeDir::new("static"));
        serve(app, 3000).await;
    };

    let origins = [
        "http://example.com".parse::<HeaderValue>().unwrap(),
        "http://localhost:3000".parse::<HeaderValue>().unwrap()
    ];

    let example1 = async {

        let app = Router::new().route("/data.json", get(json)).layer(
            CorsLayer::new()
                .allow_origin(origins)

        );
        serve(app, 4000).await;
    };

    let example2 = async {
        let app = Router::new().route("/data.json", get(json)).layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3001".parse::<HeaderValue>().unwrap())
        );
        serve(app, 4001).await;
    };

    let example2etdemi = async {
        let app = Router::new().route("/data.json", get(json2)).layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        )
            ;
        serve(app, 4005).await;
    };


    let example3 = async {
        let app = Router::new().route("/user", options(json).post(json)
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::POST, Method::OPTIONS])
                    .allow_headers([http::header::CONTENT_TYPE]),
            ));
        serve(app, 4002).await;
    };

    let example4 = async {
        let app = Router::new().route("/user", options(json)
            .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST, Method::OPTIONS])
                .allow_headers([http::header::CONTENT_TYPE]),
        ).post(accept_form)

        );
        serve(app, 4003).await;
    };

    // let example5 = async {
    //     let app = Router::new().route("/user", post(accept_error)
    //         .layer(
    //         CorsLayer::new()
    //             .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    //             .allow_methods([Method::POST, Method::OPTIONS])
    //             .allow_headers([http::header::CONTENT_TYPE]),
    //     ));
    //     serve(app, 4004).await;
    // };


    let example5 = async {
        let app = Router::new().route("/user", options(json)
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::OPTIONS, Method::POST])
                    .allow_headers([http::header::CONTENT_TYPE]),
            )).route("/user",post(accept_error).layer(
            CorsLayer::new()
            //    .allow_origin("http://localhost:4000".parse::<HeaderValue>().unwrap())

                .allow_methods([Method::OPTIONS,Method::POST])
                .allow_headers([http::header::CONTENT_TYPE])));
        serve(app, 4004).await;
    };

    let example6 = async {
        let app = Router::new().route("/user", options(json)
            .layer(
                CorsLayer::new()
                    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::OPTIONS, Method::POST])
                    .allow_headers([http::header::CONTENT_TYPE]),
            )).route("/user",post(accept_form).layer(
            CorsLayer::new()
                .allow_origin("http://localhost:4259".parse::<HeaderValue>().unwrap())

                .allow_methods([Method::OPTIONS,Method::POST])
                .allow_headers([http::header::CONTENT_TYPE])));
        serve(app, 4006).await;
    };

    tokio::join!(frontend, example1, example2, example3, example4, example5, example2etdemi, example6);
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn json() -> impl IntoResponse {
    Json(vec!["one", "two", "three"])
}
async fn json2() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "POST".parse().unwrap());
    (headers, Json(vec!["one", "two", "three"]))
}

async fn get_error()-> Result<String, StatusCode> {

    Err(StatusCode::INTERNAL_SERVER_ERROR)
    //Ok("".to_string())
}

#[derive(Serialize,Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    name: String,
    email: String,
}

async fn accept_form(Json(input): Json<Input>) -> Result<Json<Input>, StatusCode> {
    dbg!(&input);
   // Err(StatusCode::INTERNAL_SERVER_ERROR)
    Ok(Json::from(input))
}

async fn accept_error(Json(input): Json<Input>) -> Result<String, StatusCode> {
    dbg!(&input);
    Err(StatusCode::INTERNAL_SERVER_ERROR)
    //Ok("".to_string())
}