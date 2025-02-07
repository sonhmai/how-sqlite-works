use rusqlite::{ffi::sqlite3_auto_extension, Connection, Result};
use sqlite_vec::sqlite3_vec_init;
use zerocopy::IntoBytes;

fn main() -> Result<()> {
    // https://github.com/asg017/sqlite-vec/tree/main/examples/simple-rust
    unsafe {
        sqlite3_auto_extension(Some(std::mem::transmute(sqlite3_vec_init as *const ())));
    }

    let db = Connection::open_in_memory()?;
    let v: Vec<f32> = vec![0.1, 0.2, 0.3];

    let (vec_version, embedding): (String, String) = db.query_row(
        "select  vec_version(), vec_to_json(?)",
        &[v.as_bytes()],
        |x| Ok((x.get(0)?, x.get(1)?)),
    )?;

    println!("vec_version={vec_version}, embedding={embedding}");
    let items: Vec<(usize, Vec<f32>)> = vec![
        (1, vec![0.1, 0.1, 0.1, 0.1]),
        (2, vec![0.2, 0.2, 0.2, 0.2]),
        (3, vec![0.3, 0.3, 0.3, 0.3]),
        (4, vec![0.4, 0.4, 0.4, 0.4]),
        (5, vec![0.5, 0.5, 0.5, 0.5]),
    ];

    db.execute(
        "create virtual table vec_items using vec0(embedding float[4])",
        [],
    )?;
    let mut stmt = db.prepare("insert into vec_items(rowid, embedding) values (?, ?)")?;
    for item in items {
        stmt.execute(rusqlite::params![item.0, item.1.as_bytes()])?;
    }
    let query: Vec<f32> = vec![0.3, 0.3, 0.3, 0.3];
    let result: Vec<(i64, f64)> = db
        .prepare(
            r"
          SELECT
            rowid,
            distance,
            embedding
          FROM vec_items
          WHERE embedding MATCH ?1
          ORDER BY distance
          LIMIT 10 -- limit is needed in knn search
        ",
        )?
        .query_map([query.as_bytes()], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", result);

    Ok(())
}
