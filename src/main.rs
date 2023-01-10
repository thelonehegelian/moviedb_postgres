use postgres::{Client, Error, NoTls};
use std::collections::HashMap;
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
    // INSERT AND FETCH DATA

    struct Director {
        id: i32,
        name: String,
    }
    struct Movie {
        title: String,
        director_id: i32,
    }

    let mut movies = HashMap::new();
    let mut directors = HashMap::new();
    movies.insert(1, String::from("Jaws"));
    movies.insert(2, String::from("The Matrix"));
    directors.insert(1, String::from("Steven Spielberg"));
    directors.insert(2, String::from("Wachowski Brothers"));

    for (key, value) in directors {
        let director = Director {
            id: key,
            name: value.to_string(),
        };
        client.execute(
            "INSERT INTO director (id, name) VALUES ($1, $2)",
            &[&director.id, &director.name],
        )?;
    }

    for (key, value) in movies {
        let movie = Movie {
            title: value.to_string(),
            director_id: key,
        };
        client.execute(
            "INSERT INTO movie (title, director_id) ($1, $2)",
            &[&movie.title, &movie.director_id],
        )?;
    }

    Ok(())
}
