use std::env;

const LETTERS: [&str; 26] = ["ᵃ", "ᵇ", "ᶜ", "ᵈ", "ᵉ", "ᶠ", "ᵍ", "ʰ", "ⁱ", "ʲ", "ᵏ", "ˡ", "ᵐ", "ⁿ", "ᵒ", "ᵖ", "ᵠ", "ʳ", "ˢ", "ᵗ", "ᵘ", "ᵛ", "ʷ", "ˣ", "ʸ", "ᶻ"];
const GETALLEN: [&str; 10] = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];

fn main () {

    for woord in env::args().skip(1) {
        for letter in woord.chars() {
            let charcode = letter as i32;


            print!("{}", if inrange(charcode, 65, 90) {
                LETTERS[(charcode - 65) as usize].to_string()
            } else if inrange(charcode, 97, 122) {
                LETTERS[(charcode - 97) as usize].to_string()
            } else if inrange(charcode, 48, 57) {
                GETALLEN[(charcode - 48) as usize].to_string()
            } else {
                letter.to_string()
            })
        }

        print!(" ");
    }
}

fn inrange (number: i32, min: i32, max: i32) -> bool {
    number >= min && number <= max
}