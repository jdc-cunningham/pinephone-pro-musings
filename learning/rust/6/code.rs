// just writing this out

struct Sheep { naked: bool, name: &'static str } // this is weird the '

trait Animal {
  fn new(name: &'static str) -> Self;
  
  fn name(&self) -> &'static str;
  fn noise($self) -> &'static str;

  fn talk($self) {
    println!("{} says {}", self.name(), self.noise()); // interesting arg passing
  }
}

impl Sheep {
  fn is_naked(&self -> bool {
    self.naked // no return word?
  }

  fn shear(&mut self) { // interesting the mut
    if self.is_naked() {
      println!("{} is already naked...", self.name());
    } else {
      println!("{} gets a haircut!", self.name); // not a function?

      self.naked = true;
    }
  }
}

impl Animal for Sheep {
  fn new(name: &'static str) -> Sheep {
    Sheep { name: name, naked: false }
  }
  
  fn name(&self) -> &'static str { // what's going on here
    self.name
  }

  fn noise(&self) -> &'static str {
    if self.is_naked() {
      "baaaaah?" // weird
    } else {
      "baaaaah!"
    }
  }

  fn talk(&self) {
    println!("{} pause briefly... {}", self.name, self.noise());
  }
}

fn main() {
  let mut dolly: Sheep = Animal::new("Dolly"); // what is :: used for

  dolly.talk();
  dolly.shear();
  dolly.talk();
}
