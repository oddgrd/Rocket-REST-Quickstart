use rocket::{Build, Rocket};

#[database("diesel_postgres_pool")]
pub struct Db(diesel::PgConnection);

embed_migrations!();

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run_with_output(c, &mut std::io::stdout()))
        .await
        .expect("diesel migrations");

    rocket
}
