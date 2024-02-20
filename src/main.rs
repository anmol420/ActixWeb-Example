mod api;
mod repository;
mod model;

use api::task::{
    get_task,
    submit_task,
    start_task,
    fail_task,
    complete_task,
    pause_task
};
use repository::ddb::DDBRepository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use aws_sdk_dynamodb::config::BehaviorVersion;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    HttpServer::new(move || {
        let ddb_repo = DDBRepository::init(String::from("task"), config.clone());
        let ddb_data = Data::new(ddb_repo);

        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
            .service(submit_task)
            .service(start_task)
            .service(fail_task)
            .service(complete_task)
            .service(pause_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
