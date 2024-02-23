use crate::{
    file_writer::{File, Folder},
    schema::{EnumDefinition, TableDefinition},
};
use convert_case::{Case, Casing};
use std::collections::HashMap;

pub fn generate(tables: Vec<TableDefinition>, enums: Vec<EnumDefinition>) -> Vec<Folder> {
    let mut tables = table_codegen(tables);
    let mut columns = enum_codegen(enums);

    tables.append(&mut columns);

    tables
}

fn table_codegen(tables: Vec<TableDefinition>) -> Vec<Folder> {
    let tables_by_schema: HashMap<String, Vec<TableDefinition>> =
        tables.into_iter().fold(HashMap::new(), |mut acc, t| {
            let schema = t.schema.clone();
            acc.entry(schema).or_default().push(t);
            acc
        });

    let folders = tables_by_schema
        .into_iter()
        .map(|(schema, tables)| Folder {
            path: format!("./src/out/{}/", schema),
            files: tables
                .into_iter()
                .map(|t| File {
                    name: format!("{}.rs", t.name),
                    conent: handle_table(&t),
                })
                .collect(),
        })
        .collect();

    folders
}

fn handle_table(t: &TableDefinition) -> Vec<String> {
    let mut lines = vec![format!("pub struct {} {{", t.name.to_case(Case::Pascal))];
    for column in &t.columns {
        lines.push(format!("    pub {}: {},", column.name, column.data_type));
    }
    lines.push("}".to_string());
    lines
}

fn enum_codegen(enums: Vec<EnumDefinition>) -> Vec<Folder> {
    let enums_by_schema: HashMap<String, Vec<EnumDefinition>> =
        enums.into_iter().fold(HashMap::new(), |mut acc, e| {
            let schema = e.schema.clone();
            acc.entry(schema).or_default().push(e);
            acc
        });

    let folders = enums_by_schema
        .into_iter()
        .map(|(schema, enums)| Folder {
            path: format!("./src/out/{}/enums/", schema),
            files: {
                let mut files: Vec<File> = enums
                    .iter()
                    .map(|e| File {
                        name: format!("{}.rs", e.name),
                        conent: handle_enum(&e),
                    })
                    .collect();

                files.push(File {
                    name: "mod.rs".to_string(),
                    conent: enums
                        .iter()
                        .map(|e| {
                            format!(
                                "pub mod {};\npub use {}::{};\n",
                                e.name,
                                e.name,
                                e.name.to_case(Case::Pascal)
                            )
                        })
                        .collect(),
                });

                files
            },
        })
        .collect();

    folders
}

fn handle_enum(e: &EnumDefinition) -> Vec<String> {
    let mut lines = vec![format!("pub enum {} {{", e.name.to_case(Case::Pascal))];
    for variant in &e.values {
        lines.push(format!("    {},", variant.to_case(Case::Pascal)));
    }
    lines.push("}".to_string());
    lines
}
