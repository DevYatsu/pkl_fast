use super::PklTable;
use crate::PklResult;
use std::ops::Range;

mod math;

/// todo()!
///
/// Official packages support is not yet completed
pub fn import_pkg(table: &mut PklTable, pkg_uri: &str, rng: Range<usize>) -> PklResult<()> {
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
        _ => return Err((format!("Unknow Pkl Package '{pkg_uri}'"), rng)),
    };

    return Err((
        "Pkl official packages imports not yet supported!".to_owned(),
        rng,
    ));
}

pub fn amends_pkg(table: &mut PklTable, pkg_uri: &str, rng: Range<usize>) -> PklResult<()> {
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
        _ => return Err((format!("Unknow Pkl Package '{pkg_uri}'"), rng)),
    };

    return Err((
        "Pkl official packages amending not yet supported!".to_owned(),
        rng,
    ));
}
