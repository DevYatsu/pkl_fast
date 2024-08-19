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
            let src = fs::read_to_string("a.pkl").unwrap();

            let src = src.repeat(1);
            let time = Instant::now();

            let mut result = parse(&src).unwrap();

            let file = result.next().unwrap().into_inner();

            let mut doc_comment: Option<String> = None;
            for record in file {
                match record.as_rule() {
                    Rule::stmt => {
                        for stmt in record.into_inner() {
                            match stmt.as_rule() {
                                _ => {
                                    // when managing the statement
                                    // take the value inside the doc
                                    // comment to attach it to attach
                                    // it to the stmt if necessary
                                    for stmt in stmt.into_inner() {
                                        match stmt.as_rule() {
                                            Rule::COMMENT => {
                                                // println!("comment: {:?}", stmt.as_rule())
                                            }
                                            _ => {
                                                // println!("any stmt: {:?}", stmt.as_rule());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Rule::EOI => (),
                    Rule::COMMENT => {
                        for com in record.into_inner() {
                            match com.as_rule() {
                                Rule::doc_comment => {
                                    doc_comment = Some(
                                        doc_comment
                                            .map(|mut x| {
                                                x.push_str(com.as_str());
                                                x
                                            })
                                            .unwrap_or(String::from(com.as_str())),
                                    );
                                }
                                Rule::annotation => {
                                    // take care of them here
                                }
                                Rule::line_comment | Rule::multiline_comment => (),
                                _ => unreachable!(),
                            }
                        }
                    }
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
