mod utils;
mod api;

use askama::Template;
use axum::{Router, routing::get, routing::patch, http::{header, Method},
    routing::post, response::Html
};

// use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;

use dotenv::dotenv;
use std::sync::Arc;

use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub struct AppState {
    db_client: tokio::sync::Mutex<Client<tokio_util::compat::Compat<TcpStream>>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut config = Config::new();
    config.host("127.0.0.1");
    config.port(1433);
    config.database("rust_angular20");
    config.authentication(AuthMethod::sql_server("sa", "Reynald88"));
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await.expect("Failed to connect to TCP");
    tcp.set_nodelay(true).expect("REASON");
    
    let client = Client::connect(config, tcp.compat_write()).await.expect("Failed to connect to database");
    
    let app_state = Arc::new(AppState {
        db_client: tokio::sync::Mutex::new(client),
    });

    println!("Connected.....{}", "to MSSQL SERVER 2019");

    let static_files_service = ServeDir::new("assets");
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:4200".parse::<http::HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
    .allow_credentials(true);    

    let auth_routes = Router::new()
        .route("/signup", post(api::auth::register::create_user))
        .route("/signin", post(api::auth::login::create_login))
        .route("/mfa/verifytotp/:id", patch(api::auth::verifyotp::patch_verifytotp))
        .route("/products/list/:page", get(api::products::list::get_productlist))
        .route("/products/search/:page/:key", get(api::products::search::get_productsearch));

    let user_routes = Router::new()
        .route("/getuserid/:id", get(api::users::getuserid::get_userid))
        .route("/mfa/activate/:id", patch(api::auth::activatemfa::patch_activatemfa))
        .route("/getallusers", get(api::users::getallusers::get_allusers))
        .route("/updateprofile/:id", patch(api::users::updateprofile::patch_updateprofile))
        .route("/uploadpicture/:id", patch(api::users::uploadpicture::patch_uploadpicture))
        .route("/changepassword/:id", patch(api::users::changepassword::change_password))
        .layer(axum::middleware::from_fn(utils::validate_jwt));


    let app = Router::new()
        .route("/", get(root_handler))
        .nest("/api", user_routes)
        .nest("/auth", auth_routes)
        .nest_service("/assets", static_files_service)
        .layer(cors)
        .with_state(app_state);        

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();        
}

#[derive(Template)]
#[template(path = "index.html")]
struct RustTemplate<'a> {
    title: &'a str,
}

async fn root_handler() -> Html<String> {
    let template = RustTemplate {
        title: "BARCLAYS BANK",
    };
    let html_content = template.render().unwrap();
    Html(html_content)
}

