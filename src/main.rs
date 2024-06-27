use new_pkl::{Pkl, PklResult};
use std::time::Instant;

fn main() -> Result<(), (String, String)> {
    let src = "//import \"test.pkl\"

`Hello` = \"hello\"
test = 222_333.3e-4
b = true
octal = 0o1_237
hex = 0x129_EF2444443
binary = 0b1_010_10100011111101010101

multiline = \"\"\"
Although the Dodo is extinct,
the species will be remembered.
efefefefef
\"\"\"

identifier_var = multiline

bird_name = \"Common wood pigeon\"

bird {
  name = bird_name
  diet = \"Seeds\"
  taxonomy {
    species = \"Columba palumbus\"
  }
}

DATA = bird.taxonomy.species

int = 3
duration = int.min
datasize = int.gb
x = 3.4e2.ms
two = -2.ms

pigeon {
  name = \"Turtle dove\"
  extinct = false
}


parrot = (pigeon) {
  name = \"Parrot\"
}

dodo {
  name = \"Dodo\"
} {
  extinct = true
} {
  test = false
}

pigeon = new Bird {
  name = \"Pigeon\"
  lifespan = 8
  migratory = false
}

list = List()
list_with_values = List(pigeon, int, duration, two, list, List(), pigeon.lifespan)

STRING = \"test\".repeat(5)
is_start = STRING.startsWith(\"testtest\")
";

    let src = src.repeat(1);
    let time = Instant::now();

    let mut pkl = Pkl::new();
    pkl.parse(&src)
        .map_err(|(s, rng)| (s, src[rng].to_owned()))?;

    println!(
        "{}ms to parse {} chars",
        time.elapsed().as_millis(),
        src.len()
    );

    println!("{:?}", pkl);

    Ok(())
}
