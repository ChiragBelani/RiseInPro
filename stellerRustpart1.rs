fn main(){
  let x: i32 = 16;

  println!("{}",x);

  let z: String = String::from("Hello Sorobon"); // mutable string
  let y: &str = "Hello Steller"; // immutable string

  println!("{y}");
  println!("{z}");

  pub fn Event (name: String) {
    let name: String = String::from("James");
    println!("{}",name)
  }

  let e: EventforKids = EventforKids{
    name: String::from("kidsco"),
    date: String::from("01.01.2023"),
    number_of_participants: 1000,
    place: String::from("New Delhi, India")
  };




  
} 


struct EventforKids {
  name: String,
  date: String,
  number_of_participants: u32,
  place: String
}

enum Errorsforevents{
  Noevent,
  Cancelledevent,
  Eventtype
}