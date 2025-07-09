fn main() {
  //  Mutable variable
  let mut language = "Rust";
  println!("Language: {}", language);
  language = "Python";
  println!("Language: {}", language);

  // Assigning multiple variables

  let (brand, item_type) = ("Samsung", "phone");
  println!("My favourite {item_type} brand is {brand}");
}
