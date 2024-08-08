use super::{PklMember, PklTable};
use crate::PklResult;
use crate::{lexer::IsValidPkl, Pkl};
use hashbrown::HashMap;
use logos::Span;
use std::{fs, path::Path};

pub mod official;
pub mod web;

#[derive(Debug, Clone, Default)]
pub struct Importer;

impl Importer {
    pub fn construct_name_from_uri(uri: &str) -> String {
        let prefix_removed = uri
            .strip_prefix("http:|https:|pkl:|package:")
            .unwrap_or(uri);
        let suffix_removed = prefix_removed
            .strip_suffix(".pkl")
            .unwrap_or(prefix_removed);

        let mut name = String::from(suffix_removed.split('/').last().unwrap());

        if !name.is_valid_pkl_id() {
            name = name + "`";
            name.push('`');
        }

        name
    }

    pub fn import(&mut self, module_uri: &str, span: Span) -> PklResult<PklTable> {
        let mut imported_table = match module_uri {
            uri if uri.starts_with("package://") => web::import_pkg(uri, span)?,
            uri if uri.starts_with("pkl:") => official::import_pkg(uri, span)?,
            uri if uri.starts_with("https://") => web::import_http(uri, span)?,
            file_path => self.read_file_as_table(file_path, span)?,
        };

        imported_table.members.retain(|_, v| !v.is_local());

        Ok(imported_table)
    }

    /// Generates the amended table
    /// - removes the parsed local items
    /// - set all items as amended
    pub fn amends(&mut self, module_uri: &str, span: Span) -> PklResult<PklTable> {
        let mut amended_table = match module_uri {
            uri if uri.starts_with("package://") => web::amends_pkg(uri, span)?,
            uri if uri.starts_with("pkl:") => official::amends_pkg(uri, span)?,
            uri if uri.starts_with("https://") => web::amends_http(uri, span)?,
            file_path => self.read_file_as_table(file_path, span)?,
        };

        amended_table.members.retain(|_, v| {
            v.set_amended();
            !v.is_local()
        });

        Ok(amended_table)
    }

    /// Generates the extended table
    /// - removes the parsed local items
    /// - set all items as extended
    pub fn extends(&mut self, module_uri: &str, span: Span) -> PklResult<PklTable> {
        let mut extended_table = match module_uri {
            uri if uri.starts_with("package://") => web::extends_pkg(uri, span)?,
            uri if uri.starts_with("pkl:") => official::extends_pkg(uri, span)?,
            uri if uri.starts_with("https://") => web::extends_http(uri, span)?,
            file_path => self.read_file_as_table(file_path, span)?,
        };

        extended_table.members.retain(|_, v| {
            v.set_extended();
            !v.is_local()
        });

        Ok(extended_table)
    }

    fn read_file_as_table(&mut self, path_as_str: &str, span: Span) -> PklResult<PklTable> {
        // check for circular imports, amends and extends expr

        let content = self.file_content(&path_as_str, span.to_owned())?;
        let mut pkl = Pkl::new();

        pkl.parse(&content)?;
        let table = pkl.table;

        Ok(table)
    }

    fn file_content(&self, file_path: impl AsRef<Path>, span: Span) -> PklResult<String> {
        let path = file_path.as_ref();
        let file_content = fs::read_to_string(path)
            .map_err(|e| (format!("Error reading {}: {}", path.display(), e), span))?;

        Ok(file_content)
    }
}
