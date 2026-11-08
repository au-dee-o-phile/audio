use actix_web::{App, HttpServer, web::Data};
use audio::routes;
use sea_orm::Database;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool =
        Database::connect("postgres://freyjamoth:Transbian but gayer@localhost/audio").await?;

    let data = Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .service(routes::content::content_routes())
    })
    .bind(("127.0.0.1", 8888))
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
