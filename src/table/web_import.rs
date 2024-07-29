use super::PklTable;
use crate::PklResult;
use std::ops::Range;

/// todo()!
///
/// Web packages support is not yet completed
pub fn import_pkg(table: &mut PklTable, pkg_uri: &str, rng: Range<usize>) -> PklResult<()> {
    return Err(("Package imports not yet supported!".to_owned(), rng));
}

pub fn amends_pkg(table: &mut PklTable, pkg_uri: &str, rng: Range<usize>) -> PklResult<()> {
    return Err(("Package amending not yet supported!".to_owned(), rng));
}

/// todo()!
///
/// Web https packages support is not yet completed
pub fn import_https(table: &mut PklTable, pkg_uri: &str, rng: Range<usize>) -> PklResult<()> {
    return Err(("Web imports not yet supported!".to_owned(), rng));
}

pub fn amends_http(table: &mut PklTable, pkg_uri: &str, rng: Range<usize>) -> PklResult<()> {
    return Err(("Web amending not yet supported!".to_owned(), rng));
}
