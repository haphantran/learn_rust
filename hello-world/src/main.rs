fn main() {
  // This is the a simple program to learn about Basic formatting
  println!("Hello, world!");

  // Multiple Placeholders
  println!("{} is a {}.", "Apple", "fruit");

  // Positional placeholder
  println!("{0} is a {1}. My favourite {1} is {0}.", "Apple", "fruit");

  // named placholders
  println!("{name} is a {type}. My favourite {type} is {name}.", name="Samsung", type="phone");

  //Placeholder Traits: In the placeholder for binary, hexadecimal, or octal respectively and in the value specify the number.

  println!(
    "Number : {number} \nBinary: {number:b} Hexadecimal: {number:x} Octal: {number:o}",
    number = 10
  );

  //Basic math
  println!("{} + {} = {}", 23, 10, 23 + 10);

  // debug trait: multiple values
  println!("{:?}", ("This is a Rust Course", 101, "hello"));
}
