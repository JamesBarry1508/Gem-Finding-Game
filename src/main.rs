use std::fmt::{self, Display};
use std::io;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, FromPrimitive)]
enum Gem {
    Diamond = 1,
    Ruby,
    Saphire,
    Topaz,
    Onyx,
    Jade,
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gem::Diamond => write!(f, "Diamond"),
            Gem::Ruby => write!(f, "Ruby"),
            Gem::Saphire => write!(f, "Saphire"),
            Gem::Topaz => write!(f, "Topaz"),
            Gem::Onyx => write!(f, "Onyx"),
            Gem::Jade => write!(f, "Jade"),
        }
    }
}

fn main() {
    let mut map = [[0; 5]; 5];

    map[4][2] = 1;
    map[1][2] = 2;
    map[0][2] = 3;
    map[3][3] = 4;
    map[1][4] = 5;
    map[4][2] = 6;

    for row in map {
        println!("{:?}", row);
    }

    println!("Enter X and Y coordinate: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        println!("2 inputs needed");
        return;
    }

    let (x, y) = match (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
        (Ok(x), Ok(y)) => (x, y),
        _ => {
            println!("Error");
            return;
        }
    };

    let mut found: Vec<Gem> = Vec::new();
    let data = map[x as usize][y as usize];
    found.push(Gem::from_u8(data).expect("No Gem"));
    println!("{found:?}");
}
