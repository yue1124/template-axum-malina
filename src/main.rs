use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, get_service},
    Router,
};
use clap::Parser;
use std::process::Command;
use tower_http::{services::ServeDir, trace::TraceLayer};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    /// true if for production
    #[clap(short, long)]
    production: bool,
}

async fn home() -> Html<&'static str> {
    Html(include_str!("../home.html"))
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "bigyue=debug,tower_http=debug")
    }

    tracing_subscriber::fmt::init();

    // read config
    let config = Config::parse();
    println!("! {:?}", config);

    // check node
    if let Ok(output) = Command::new("node").arg("--version").output() {
        if !output.status.success() {
            panic!("{}", String::from_utf8(output.stderr).unwrap());
        }
        println!(
            "! node {}",
            String::from_utf8(output.stdout).unwrap().trim()
        );
    } else {
        panic!("nodejs is not installed correctly")
    }

    // check npm
    let npm_clijs_path;
    if let Ok(output) = Command::new("node").arg("utils/where_npm.js").output() {
        if !output.status.success() {
            panic!("{}", String::from_utf8(output.stderr).unwrap());
        } else {
            npm_clijs_path = String::from_utf8(output.stdout).unwrap().trim().to_string();
        }
    } else {
        panic!("nodejs is not installed correctly")
    }

    // check npx
    let npx_clijs_path;
    if let Ok(output) = Command::new("node").arg("utils/where_npx.js").output() {
        if !output.status.success() {
            panic!("{}", String::from_utf8(output.stderr).unwrap());
        } else {
            npx_clijs_path = String::from_utf8(output.stdout).unwrap().trim().to_string();
        }
    } else {
        panic!("nodejs is not installed correctly")
    }

    if let Ok(output) = Command::new("node")
        .arg(&npm_clijs_path)
        .arg("--version")
        .output()
    {
        if !output.status.success() {
            panic!("{}", String::from_utf8(output.stderr).unwrap());
        }
        println!("! npm {}", String::from_utf8(output.stdout).unwrap().trim());
    } else {
        panic!("npm is not installed correctly")
    }

    let curr_dir = std::env::current_dir().ok().unwrap();
    let malina_dir = curr_dir.join("malina");
    // check malina
    if !malina_dir.is_dir() {
        if let Ok(output) = Command::new("node")
            .arg(&npx_clijs_path)
            .arg("create-malina")
            .arg("malina")
            .output()
        {
            if !output.status.success() {
                panic!(
                    "{}{}",
                    String::from_utf8(output.stdout).unwrap(),
                    String::from_utf8(output.stderr).unwrap()
                );
            }
            println!("{}", String::from_utf8(output.stdout).unwrap().trim());
            std::fs::remove_file(malina_dir.join(".gitignore")).ok();
            std::fs::remove_file(malina_dir.join("README.md")).ok();
            std::fs::remove_dir_all(malina_dir.join("public")).ok();
            // modify rollup.config.js
            std::fs::write(
                malina_dir.join("rollup.config.js"),
                include_str! {"../utils/rollup.config.js"},
            )
            .ok();
            std::fs::write(
                malina_dir.join("src/App.xht"),
                include_str! {"../utils/App.xht"},
            )
            .ok();
        } else {
            panic!("malina installed failed")
        }
    }
    // run malina build
    if let Ok(output) = Command::new("node")
        .arg(&npx_clijs_path)
        .arg("rollup")
        .arg("-c")
        .env(
            "CARGO_MALINA_DEV",
            if config.production { "false" } else { "true" },
        )
        .current_dir(&malina_dir)
        .output()
    {
        if !output.status.success() {
            panic!(
                "{}{}",
                String::from_utf8(output.stdout).unwrap(),
                String::from_utf8(output.stderr).unwrap()
            );
        }
        println!("{}", String::from_utf8(output.stdout).unwrap().trim());
    } else {
        panic!("malina installed failed")
    }

    // build our application with a single route
    let app = Router::new()
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
        .route("/", get(home))
        .layer(TraceLayer::new_for_http());

    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on http://{}", addr);
    // run it with hyper on localhost:3000
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
