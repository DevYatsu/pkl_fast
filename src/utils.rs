// Helper macro to count arguments
#[macro_export]
macro_rules! count_args {
    ($($arg_index:tt),*) => {
        <[()]>::len(&[$(count_args!(@single $arg_index)),*])
    };
    (@single $arg_index:tt) => { () };
}

#[macro_export]
macro_rules! generate_method {
    ($name:expr,$args:expr; $($arg_index:tt : $arg_type:ident),+; $action:expr; $range:expr) => {{
        use crate::count_args;

        let name: &str = $name;
        let number_of_args: usize = count_args!($($arg_index),+);
        let args: &Vec<PklValue> = $args;

        if args.len() != number_of_args {
            return Err((
                format!(
                    "Method '{}' expects exactly {} argument(s)",
                    name, number_of_args
                ),
                $range,).into());
        }

        $(
            if stringify!($arg_type) == "Number" {
                if args[$arg_index].get_type() != "Float" && args[$arg_index].get_type() != "Int" {
                    return Err((
                        format!(
                            "{} method expects argument at index {} to be of type Number, but found {}",
                            name, $arg_index, args[$arg_index].get_type()
                        ),
                        $range).into());
                }
            } else if args[$arg_index].get_type() != stringify!($arg_type) {
                return Err((
                    format!(
                        "{} method expects argument at index {} to be of type {}, but found {}",
                        name, $arg_index, stringify!($arg_type), args[$arg_index].get_type()
                    ),
                    $range).into());
            }
        )+

        let args_tuple = (
            $(
                match &args[$arg_index] {
                    PklValue::$arg_type(value) => value.to_owned(),
                    _ => return Err((
                        format!(
                            "{} method expects argument at index {} to be of type {}, but found {}",
                            name, $arg_index, stringify!($arg_type), args[$arg_index].get_type()
                        ),
                        $range).into()),
                }
            ),+
        );

        $action(args_tuple).map_err(|e: (String, Range<usize>)| e.into())
    }};
    ($name:expr,$args:expr; $action:expr; $range:expr) => {{
        let name: &str = $name;
        let number_of_args: usize = 0;
        let args: &Vec<PklValue> = $args;

        if args.len() != number_of_args {
            return Err((
                format!(
                    "Method '{}' expects 0 argument",
                    name
                ),
                $range,).into());
        }

        $action
    }};

    ($name:expr, $args:expr; Numbers: $args_number:expr; $action:expr; $range:expr) => {{
        // Case only useful when the method takes several Number arguments

        let name: &str = $name;
        let number_of_args: usize = $args_number;
        let args: &Vec<PklValue> = $args;
        if args.len() != number_of_args {
            return Err((
                format!(
                    "Method '{}' expects exactly {} argument(s)",
                    name, number_of_args
                ),
                $range,).into());
        }



        let mut args_tuple: [f64; $args_number] = [0.0; $args_number];

        for arg_number in 0..=number_of_args {
            if args[arg_number].get_type() != "Float" && args[arg_number].get_type() != "Int" {
                return Err((
                    format!(
                        "{} method expects argument at index {} to be of type Number, but found {}",
                        name, arg_number, args[arg_number].get_type()
                    ),
                    $range).into());
            }

            args_tuple[arg_number] = match &args[arg_number] {
                PklValue::Float(value) => *value,
                PklValue::Int(value) => *value as f64,
                _ => return Err((
                    format!(
                        "{} method expects argument at index {} to be of type Number, but found {}",
                        name, arg_number, args[arg_number].get_type()
                    ),
                    $range).into()),
            };
        }
        $action(args_tuple)
    }};

}
