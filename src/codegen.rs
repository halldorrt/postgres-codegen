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
    let mut lines = vec![format!("pub struct {} {{", t.name.to_case(Case::Pascal))];
    for column in &t.columns {
        lines.push(format!("    pub {}: {},", column.name, column.data_type));
    }
    lines.push("}".to_string());
    lines
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

fn mod_and_use(mod_name: &str) -> String {
    format!("mod {mod_name};\npub use {mod_name}::*;")
}

fn handle_enum(e: &EnumDefinition) -> Vec<String> {
    let mut lines = vec![format!("pub enum {} {{", e.name.to_case(Case::Pascal))];
    for variant in &e.values {
        lines.push(format!("    {},", variant.to_case(Case::Pascal)));
    }
    lines.push("}".to_string());
    lines
}
