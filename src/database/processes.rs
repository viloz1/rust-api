use sqlx::{SqlitePool, Error};

pub async fn populate(pool: &SqlitePool) -> Result<(),Error> {
    sqlx::query(
		r#"
        CREATE TABLE IF NOT EXISTS Processes (
        Name text,
        Id integer primary key autoincrement,
        UNIQUE(Id)
        );"#,
	)
	.execute(pool)
	.await?;

    sqlx::query("insert or ignore into Processes (Name) values ($1)")
		.bind("new process")
		.execute(pool)
		.await?;

    return Ok(())

}