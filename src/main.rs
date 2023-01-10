use postgres::{Client, Error, NoTls};
fn main() -> Result<(), Error> {
    // postgres link format: postgresql://[user[:password]@][netloc][:port][/dbname][?param1=value1&...]
    let mut client = Client::connect("postgresql://nobody:password@localhost:5243/library", NoTls)?;

    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS director (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL 
    )",
    )?;
    client.batch_execute(
        "CREATE TABLE IF NOT EXISTS movie (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        director_id INTEGER NOT NULL REFERENCES director
    )",
    )?;

    Ok(())
}
