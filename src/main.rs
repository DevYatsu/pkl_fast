use new_pkl::Pkl;
use std::time::Instant;

fn main() -> Result<(), (String, String)> {
    let src = "
module com.animals.test

// amends \"x.pkl\"
// amends \"test.pkl\"

extends \"x.pkl\"

import \"test.pkl\" as test_import
import \"test.pkl\"

zero = 2

typealias EmailAddress = String(matches(Regex(\".+@.+\")))
typealias EmailList = List<EmailAddress>
typealias StringMap<Value> = Map<String, Value>
    | EmailList
    | EmailAddress

// Define a type alias over several lines
typealias ComplexType =
        OptionOne |
        OptionTwo |
        OptionThree |
        OptionFour |
        OptionFive

abstract class Bird {
  name: String
}

class ParentBird extends Bird {
  kids: List<String>
}

// Define a class with fields declared over several lines
class Bird2 {
  species: String
  wingspan: Float
  canFly: Boolean

  /// Age of the bird in years
  age: Int?

  /// The diet of the bird, which can be one of several options
  diet:
    \"Seeds\"
    | \"Berries\" |
    \"Insects\" |
    \"Nectar\"

  /// The migration duration in hours
  migrationDuration:
    Duration
}

bird000 = new Bird {
    name = \"string\"
}

`Hello`: String = \"Hello\"
testtt: Float = 222_333.3e-4
b: Boolean = true
octal = 0o1_237
hex = 0x129_EF2444443
binary = 0b1_010_10100011111101010101

num = \"122222222\".toInt()
$string_bool = \"false\".toBoolean()
title: String = \"myTitle\".capitalize()

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

pigeon2: Bird = new {
  name = \"Pigeon\"
// lifespan = 8
// migratory = false
}

list: List<Mapping<String, Number>> | List<Int> = List()
list_with_values = List(pigeon, int, duration, two, list, List(), pigeon.name)

STRING = \"test\".repeat(5)
is_start = STRING.startsWith(\"testtest\")

s = 5.min.toUnit(\"s\")
";

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
