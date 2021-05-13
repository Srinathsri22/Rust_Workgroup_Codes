fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice","Bob");
    println!("{subject}! {verb}! {object}!", object = "the lazy dog", subject = "the quick brown fox", verb = "jumps over");
    println!("{} of {:b} people know binary, the other half doesn't",1,2);
    println!("{num:>width$}", num = 10, width = 20);
    println!("{num:>0width$}", num = 10, width = 6);
    println!("My name is {0},{1} {0}", "James", "Bond");
  //  #[allow(dead_code)]
    //struct Structure(i32);
   // println!("This is a structure",Structure(3));
   let pi = 3.141592; 
   println!("PI is roughly {}",pi);
}