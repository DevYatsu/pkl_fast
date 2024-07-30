use new_pkl::Pkl;
use std::{fs, time::Instant};

fn main() -> Result<(), (String, String)> {
    let src = fs::read_to_string("main.pkl").unwrap();

    let src = src.repeat(1);
    let time = Instant::now();

    let mut pkl = Pkl::new();
    pkl.parse(&src)
        .map_err(|(s, rng)| (s, src[rng].to_owned()))?;

    // for stmt in pkl.generate_ast(&src).unwrap() {
    //     println!("{stmt:?}",);
    // }

    println!(
        "{}ms to parse {} chars",
        time.elapsed().as_millis(),
        src.len()
    );

    println!("{:?}", pkl);

    Ok(())
}
