use crate::PklResult;
use crate::PklTable;
use logos::Span;

/// todo()!
///
/// Web packages support is not yet completed
pub fn import_pkg(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    return Err(("Package imports not yet supported!".to_owned(), span));
}

pub fn amends_pkg(table: &mut PklTable, pkg_uri: &str, span: Span) -> PklResult<()> {
    return Err(("Package amending not yet supported!".to_owned(), span));
}
pub fn extends_pkg(table: &mut PklTable, pkg_uri: &str, span: Span) -> PklResult<()> {
    return Err(("Package extending not yet supported!".to_owned(), span));
}

/// todo()!
///
/// Web https packages support is not yet completed
pub fn import_https(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    return Err(("Web imports not yet supported!".to_owned(), span));
}

pub fn amends_http(table: &mut PklTable, pkg_uri: &str, span: Span) -> PklResult<()> {
    return Err(("Web amending not yet supported!".to_owned(), span));
}
pub fn extends_http(table: &mut PklTable, pkg_uri: &str, span: Span) -> PklResult<()> {
    return Err(("Web extending not yet supported!".to_owned(), span));
}
