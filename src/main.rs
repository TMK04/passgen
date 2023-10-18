use cli_clipboard::{ClipboardContext, ClipboardProvider};
use passwords::PasswordGenerator;
use passwords::analyzer;
use passwords::scorer;

const PG: PasswordGenerator = PasswordGenerator {
  length: 20,
  numbers: true,
  lowercase_letters: true,
  uppercase_letters: true,
  symbols: true,
  spaces: true,
  exclude_similar_characters: false,
  strict: true,
};

fn main() {
  let password: String = PG.generate_one().unwrap();
  let score: f64 = scorer::score(&analyzer::analyze(&password));
  println!("{} {}", password, score);
  
  let mut ctx = ClipboardContext::new().unwrap();
  ctx.set_contents(password.to_owned()).unwrap();
}