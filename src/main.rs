#![allow(unused)]
trait Animal {
    fn make_sound(&self) -> &str;
}
struct Cat;
struct Dog;

impl Animal for Cat {
    fn make_sound(&self) -> &str {
        "miaow"
    }
}
impl Animal for Dog {
    fn make_sound(&self) -> &str {
        "woof"
    }
}

enum PetType {
    Catly,
    Dogly,
}

impl Animal for PetType {
    fn make_sound(&self) -> &str {
        match &self {
            PetType::Catly => Cat.make_sound(),
            PetType::Dogly => Dog.make_sound(),
        }
    }
}

// we can use either 'impl Animal' or 'PetType' = static dispatch
fn pet_setup(input: &str) -> impl Animal {
    match input {
        "cat" => PetType::Catly,
        "dog" => PetType::Dogly,
        _ => panic!("bad pet name"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter your pet type");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let my_pet = input.trim();
    let petx = pet_setup(my_pet);
    dbg!(petx.make_sound());
    Ok(())
}
