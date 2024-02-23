use std::time::{SystemTime, UNIX_EPOCH};

use codegen::{
    codegen::enum_codegen,
    file_writer::{FileSystemWriter, FileWriter},
    schema::{DbSchema, PostgresSchema},
};

#[tokio::main]
async fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let schema = PostgresSchema::new().await;

    let (tables, columns, enums) = tokio::join!(
        schema.get_tables(),
        schema.get_columns(),
        schema.get_enums()
    );

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    println!("We print the tables:\n");
    println!("{:?}", tables);

    println!("We print the columns:\n");
    println!("{:?}", columns);

    println!("We print the enums:\n");
    println!("{:?}", enums);

    let folders = enum_codegen(enums);

    let writer = FileSystemWriter;
    writer.write(folders);

    println!("Time elapsed: {:?}", end - start);
}
