use postgres::{Client, Error, NoTls};
fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://nobody:password@localhost:5243/postgres",
        NoTls,
    )?;
    Ok(())
}
