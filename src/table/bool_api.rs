use crate::{generate_method, PklResult, PklValue};
use std::ops::Range;

/// Based on v0.26.0
pub fn match_bool_methods_api<'a, 'b>(
    bool_value: bool,
    fn_name: &'a str,
    args: Vec<PklValue<'b>>,
    range: Range<usize>,
) -> PklResult<PklValue<'b>> {
    match fn_name {
        "xor" => {
            // if args.len() != 1 {
            //     return Err((
            //         format!("Boolean expects 'xor' method to take exactly 1 argument"),
            //         range,
            //     ));
            // }

            // if let Some(other_bool) = args[0].as_bool() {
            //     return Ok((bool_value ^ other_bool).into());
            // } else {
            //     return Err((
            //         format!("1st argument of method 'xor' is expected to be a boolean, argument is of type: `{}`", args[0].get_type()),
            //         range,
            //     ));
            // };

            generate_method!(
                "xor", &args;
                0: Bool;
                |other_bool: bool| {
                        Ok((bool_value ^ other_bool).into())
                };
                range
            )
        }
        "implies" => {
            generate_method!(
                "implies", &args;
                0: Bool;
                |other_bool: bool| {
                        Ok((!bool_value || other_bool).into())
                };
                range
            )
        }
        _ => {
            return Err((
                format!("Boolean does not possess {} method", fn_name),
                range,
            ))
        }
    }
}
