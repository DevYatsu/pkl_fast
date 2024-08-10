use new_pkl::{
    pest::{parse, Rule},
    Pkl, PklError,
};
use std::{env::args, fs, time::Instant};

fn main() -> Result<(), (String, String, Option<String>)> {
    let args = args().into_iter().collect::<Vec<_>>();

    match args.get(1) {
        Some(_) => {
            let src = fs::read_to_string("main.pkl").unwrap();

            let src = src.repeat(1000);
            let time = Instant::now();

            let mut pkl = Pkl::new();

            let ast = pkl.generate_ast(&src).map_err(|e: PklError| {
                (
                    e.msg().to_owned(),
                    src[e.span().unwrap(/* safe */)].to_owned(),
                    e.file_name().to_owned(),
                )
            })?;
            // pkl.parse(&src).map_err(|e: PklError| {
            //     (
            //         e.msg().to_owned(),
            //         src[e.span().unwrap(/* safe */)].to_owned(),
            //         e.file_name().to_owned(),
            //     )
            // })?;

            // for stmt in pkl.generate_ast(&src).unwrap() {
            //     println!("{stmt:?}",);
            // }

            println!(
                "{}ms to parse {} chars",
                time.elapsed().as_millis(),
                src.len()
            );

            println!("{:?}", pkl);
        }
        None => {
            let src = fs::read_to_string("base.pkl").unwrap();

            let src = src.repeat(1);
            let time = Instant::now();

            let mut result = parse(&src).unwrap();

            let file = result.next().unwrap().into_inner();
            for record in file {
                match record.as_rule() {
                    Rule::stmt => {
                        for stmt in record.into_inner() {
                            match stmt.as_rule() {
                                Rule::amends => {}
                                Rule::import => {}
                                Rule::extends => {}
                                Rule::module => {}
                                Rule::typealias => {}
                                Rule::with_annotation => {
                                    println!("{:?}", stmt.as_str())
                                }
                                Rule::with_doc_comment => {
                                    println!("{:?}", stmt.as_str())
                                }
                                Rule::function => {}
                                Rule::class => {}
                                Rule::property => {}
                                Rule::expr => {}
                                _ => unreachable!(),
                            }
                        }
                    }
                    Rule::EOI => (),
                    _ => unreachable!(),
                }
            }

            println!(
                "{}ms to parse {} chars",
                time.elapsed().as_millis(),
                src.len()
            );
        }
    };

    Ok(())
}
