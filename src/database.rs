use atom_syndication::Entry;
use rusqlite::{params, Connection, Result};

pub fn init_db(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;

    conn.execute(
        "create table if not exists jobs (
            id text primary key
        )",
        [],
    )?;

    Ok(conn)
}

pub fn is_table_empty(conn: &Connection) -> Result<bool> {
    let table_count: usize = conn
        .prepare("select count(*) from jobs")?
        .query_row(params![], |row| row.get(0))
        .expect("Can't get the number of rows in jobs table.");

    Ok(table_count == 0)
}

pub fn exists(conn: &Connection, entry: &Entry) -> Result<bool> {
    let exists: usize = conn
        .prepare("select exists(select 1 from jobs where id=?)")?
        .query_row(params![&entry.id], |row| row.get(0))
        .expect("Can't get whether the giving entry exists in the database.");

    Ok(exists == 1)
}

pub fn insert(conn: &Connection, entry: &Entry) -> Result<usize> {
    conn.execute("INSERT INTO jobs (id) values (?)", &[&entry.id])
}
