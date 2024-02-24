use crate::{
    file_writer::{File, Folder},
    schema::{EnumDefinition, TableDefinition},
};
use convert_case::{Case, Casing};
use std::collections::HashMap;

pub fn generate(tables: Vec<TableDefinition>, enums: Vec<EnumDefinition>) -> Vec<Folder> {
    let tables = tables_by_schema(tables);
    let enums = enums_by_schema(enums);

    let mut all_schemas = tables.keys().chain(enums.keys()).collect::<Vec<_>>();
    all_schemas.sort();
    all_schemas.dedup();

    let mod_file = Folder {
        path: "./src/out/".into(),
        files: vec![File {
            name: "mod".into(),
            extension: "rs".into(),
            content: all_schemas
                .iter()
                .map(|s| format!("pub mod {s};"))
                .collect(),
        }],
    };

    let mut folders = vec![mod_file];

    for (schema, mut files) in tables {
        let path = format!("./src/out/{}/", schema);

        let mut mod_file = File {
            name: "mod".into(),
            extension: "rs".into(),
            content: files
                .iter()
                .map(|f| mod_and_use(&f.name))
                .collect::<Vec<String>>(),
        };
        if enums.contains_key(&schema) {
            mod_file.content.push("pub mod enums;".into());
            mod_file.content.push("pub use enums::*;".into());
        }
        files.push(mod_file);

        folders.push(Folder { path, files })
    }

    for (schema, enums) in enums {
        folders.push(Folder {
            path: format!("./src/out/{}/enums/", schema),
            files: enums,
        });
    }

    folders
}

fn mod_and_use(mod_name: &str) -> String {
    format!("mod {mod_name};\npub use {mod_name}::*;")
}

/// Generate files for each table and group them by schema.
fn tables_by_schema(tables: Vec<TableDefinition>) -> HashMap<String, Vec<File>> {
    tables.into_iter().fold(HashMap::new(), |mut map, t| {
        let content = handle_table(&t);
        map.entry(t.schema).or_default().push(File {
            name: t.name,
            extension: "rs".into(),
            content,
        });
        map
    })
}

fn handle_table(t: &TableDefinition) -> Vec<String> {
    let column_types = &column_types();

    let mut lines = vec![format!("pub struct {} {{", t.name.to_case(Case::Pascal))];
    for column in &t.columns {
        let mut data_type: String = if column.data_type == "USER-DEFINED" {
            column.udt_name.to_case(Case::Pascal)
        } else if column.data_type == "ARRAY" {
            // TODO: this doesn't work with user defined types
            format!("Vec<{}>", column.udt_name.to_case(Case::Pascal))
        } else {
            column_types
                .get(column.data_type.as_str())
                .expect(&format!(
                    "Unknown column data type {}",
                    column.data_type.as_str()
                ))
                .to_string()
        };

        if column.is_nullable {
            data_type = format!("Option<{}>", data_type);
        }

        lines.push(format!("    pub {}: {},", column.name, data_type));
    }
    lines.push("}".to_string());
    lines
}

fn column_types() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("boolean", "bool");
    map.insert("char", "String"); // TODO: is this correct?
    map.insert("character", "String");
    map.insert("character varying", "String");
    map.insert("text", "String");
    map.insert("smallint", "i16");
    map.insert("integer", "i32");
    map.insert("bigint", "i64");
    map.insert("numeric", "String"); // TODO: How is this handled?
    map.insert("double precision", "f64");
    map.insert("date", "chrono::NaiveDate");
    map.insert(
        "timestamp without time zone",
        "chrono::DateTime<chrono::Utc>",
    );
    map.insert("timestamp with time zone", "chrono::DateTime<chrono::Utc>");
    map.insert("uuid", "uuid::Uuid");
    map.insert("json", "serde_json::Value");
    map.insert("jsonb", "serde_json::Value");
    map
}

/// Takes enum definitions and returns a map, where the key is the schema name
/// and the value is a vector of enum files, one for each enum.
fn enums_by_schema(enums: Vec<EnumDefinition>) -> HashMap<String, Vec<File>> {
    let mut enums_by_schema: HashMap<String, Vec<File>> =
        enums.into_iter().fold(HashMap::new(), |mut map, e| {
            let content = handle_enum(&e);
            map.entry(e.schema).or_default().push(File {
                name: e.name,
                extension: "rs".into(),
                content,
            });
            map
        });

    for (_, enums) in &mut enums_by_schema {
        enums.push(File {
            name: "mod".into(),
            extension: "rs".into(),
            content: enums.iter().map(|e| mod_and_use(&e.name)).collect(),
        });
    }

    enums_by_schema
}

fn handle_enum(e: &EnumDefinition) -> Vec<String> {
    let mut lines = vec![format!("pub enum {} {{", e.name.to_case(Case::Pascal))];
    for variant in &e.values {
        lines.push(format!("    {},", variant.to_case(Case::Pascal)));
    }
    lines.push("}".to_string());
    lines
}
