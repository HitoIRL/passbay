mod routes;
mod utils;
mod models;
mod errors;

use chrono::Local;
use dotenvy::dotenv;
use fern::colors::{ColoredLevelConfig, Color};
use log::error;
use poem::{Route, Server, listener::TcpListener, EndpointExt};
use routes::passwords;
use sqlx::MySqlPool;
use utils::get_env_var;

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::BrightBlue)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            let date = Local::now();

            out.finish(format_args!(
                "{}[{} {} {}] {}\x1B[0m",
                format_args!(
                    "\x1B[{}m",
                    colors.get_color(&record.level()).to_fg_str()
                ),
                date.format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(why) = setup_logger() {
        eprintln!("Failed to setup logger: {why}");
    }

    dotenv().ok();

    let db_url = get_env_var("DATABASE_URL");
    let pool = MySqlPool::connect(&db_url).await;

    match pool {
        Ok(pool) => {
            let app = Route::new()
                .nest("/passwords", passwords::register_routes())
                .data(pool);

            let server = Server::new(TcpListener::bind("127.0.0.1:3000"))
                .run(app)
                .await;

            if let Err(why) = server {
                eprintln!("Failed to start API: {why}");
            }
        }
        Err(why) => error!("Failed to connect to database: {why}")
    }
}
