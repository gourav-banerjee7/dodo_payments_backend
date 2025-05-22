use dodo_payments_backend::create_app;
use std::process::Command;

pub async fn spawn_app() -> axum::Router {
    // Ensure `.env.test` is loaded
    dotenvy::from_filename(".env.test").ok();

    // Wait for DB healthcheck to pass (if needed)
    for _ in 0..5 {
        if is_db_ready() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    create_app().await
}

fn is_db_ready() -> bool {
    Command::new("pg_isready")
        .args(["-h", "localhost", "-p", "5433", "-U", "postgres"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
