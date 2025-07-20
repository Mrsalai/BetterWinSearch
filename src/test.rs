use rusqlite::Connection;

fn main()
{
    let dbcon  = Connection::open("index.db3").unwrap();
    dbcon.execute("
    CREATE TABLE IF NOT EXISTS apps (
        name TEXT,
        location TEXT,
        imgae BLOB

    )",()).unwrap();

    dbcon.execute("INSERT INTO apps (name, location) VALUES(?1, ?2)" ,("yes","c"));
}