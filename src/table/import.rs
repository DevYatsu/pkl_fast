use super::PklTable;
use crate::{lexer::IsValidPkl, Pkl};
use crate::{PklResult, PklValue};
use hashbrown::{HashMap, HashSet};
use logos::Span;
use std::{fs, path::Path};

pub mod official;
pub mod web;

#[derive(Debug, Clone, Default)]
pub struct Importer {
    currently_importing: HashSet<String>,
}

pub type PklModule = HashMap<String, PklValue>;

impl Importer {
    pub fn new() -> Self {
        Importer {
            currently_importing: HashSet::new(),
        }
    }

    pub fn construct_name_from_uri(uri: &str, span: Span) -> PklResult<String> {
        let prefix_removed = uri
            .strip_prefix("http:|https:|pkl:|package:")
            .unwrap_or(uri);
        let suffix_removed = prefix_removed
            .strip_suffix(".pkl")
            .unwrap_or(prefix_removed);

        let name = suffix_removed.split('/').last().unwrap();

        if !name.is_valid_pkl_id() {
            return Err((
                format!("Cannot extract a valid name out of uri '{uri}'"),
                span,
            )
                .into());
        }

        Ok(name.to_owned())
    }

    pub fn import(&mut self, module_uri: &str, span: Span) -> PklResult<PklTable> {
        match module_uri {
            uri if uri.starts_with("package://") => return web::import_pkg(uri, span),
            uri if uri.starts_with("pkl:") => return official::import_pkg(uri, span),
            uri if uri.starts_with("https://") => return web::import_https(uri, span),
            file_path => self.import_file(file_path, span),
        }
    }

    pub fn import_file(&mut self, path_as_str: &str, span: Span) -> PklResult<PklTable> {
        if self.currently_importing.iter().any(|p| {
            Importer::are_same_file(p.as_str(), path_as_str, span.to_owned()).unwrap_or(false)
        }) {
            return Err((
                format!("Circular import detected for file: {:?}", path_as_str),
                span,
            )
                .into());
        }

        self.currently_importing.insert(path_as_str.to_owned());

        let content = self.file_content(&path_as_str, span.to_owned())?;
        let mut pkl = Pkl::new();
        pkl.parse(&content)?;
        let table = pkl.table;

        self.currently_importing.remove(path_as_str);

        Ok(table)
    }

    fn file_content(&self, file_path: impl AsRef<Path>, span: Span) -> PklResult<String> {
        let path = file_path.as_ref();
        let file_content = fs::read_to_string(path)
            .map_err(|e| (format!("Error reading {}: {}", path.display(), e), span))?;

        Ok(file_content)
    }

    fn are_same_file<P: AsRef<Path>>(path1: P, path2: P, span: Span) -> PklResult<bool> {
        let canonical_path1 = fs::canonicalize(path1).map_err(|e| (e.to_string(), span.clone()))?;
        let canonical_path2 = fs::canonicalize(path2).map_err(|e| (e.to_string(), span))?;
        Ok(canonical_path1 == canonical_path2)
    }
}
