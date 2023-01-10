use postgres::{Client, Error, NoTls};
fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres",
        NoTls,
    )?;
    Ok(())
}
