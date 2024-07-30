use crate::PklResult;
use crate::PklTable;
use logos::Span;

/// todo()!
///
/// Web packages support is not yet completed
pub fn import_pkg(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    return Err(("Package imports not yet supported!".to_owned(), span).into());
}

pub fn amends_pkg(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    return Err(("Package amending not yet supported!".to_owned(), span).into());
}
pub fn extends_pkg(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    return Err(("Package extending not yet supported!".to_owned(), span).into());
}

/// todo()!
///
/// Web https packages support is not yet completed
pub fn import_http(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    return Err(("Web imports not yet supported!".to_owned(), span).into());
}

pub fn amends_http(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    return Err(("Web amending not yet supported!".to_owned(), span).into());
}
pub fn extends_http(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    return Err(("Web extending not yet supported!".to_owned(), span).into());
}
