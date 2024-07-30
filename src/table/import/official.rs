use crate::PklResult;
use crate::PklTable;
use logos::Span;

mod math;

/// todo()!
///
/// Official packages support is not yet completed
pub fn import_pkg(pkg_uri: &str, span: Span) -> PklResult<PklTable> {
    match pkg_uri {
        "pkl:Benchmark" => {}
        "pkl:DocPackageInfo" => {}
        "pkl:DocsiteInfo" => {}
        "pkl:EvaluatorSettings" => {}
        "pkl:json" => {}
        "pkl:jsonnet" => {}
        "pkl:math" => {}
        "pkl:platform" => {}
        "pkl:Project" => {}
        "pkl:protobuf" => {}
        "pkl:reflect" => {}
        "pkl:release" => {}
        "pkl:semver" => {}
        "pkl:settings" => {}
        "pkl:shell" => {}
        "pkl:test" => {}
        "pkl:xml" => {}
        "pkl:yaml" => {}
        _ => return Err((format!("Unknow Pkl Package '{pkg_uri}'"), span).into()),
    };

    return Err((
        "Pkl official packages imports not yet supported!".to_owned(),
        span,
    )
        .into());
}

pub fn amends_pkg(table: &mut PklTable, pkg_uri: &str, span: Span) -> PklResult<()> {
    return Err((
        "Pkl official packages amending not yet supported!".to_owned(),
        span,
    )
        .into());
}
pub fn extends_pkg(table: &mut PklTable, pkg_uri: &str, span: Span) -> PklResult<()> {
    return Err((
        "Pkl official packages extending not yet supported!".to_owned(),
        span,
    )
        .into());
}
