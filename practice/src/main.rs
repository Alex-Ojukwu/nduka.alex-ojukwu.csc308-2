fn main() {
    // let user_option: i32 = 1;

    // match user_option {
    //     1 => println!("You ordered rice!"),
    //     2 => println!("You ordered beef"),
    //     _ => println!("Invalid Option"),
    // }

   // fn multiplication(value: i32) -> i32 {
      //  value * 5
    //}

    // multiplication(2)

    macro_rules! my_macro {
        ($arg:expr) => {
            println!("you entered: {}", $arg);  
        
        };
    };
}
  my_macro!();  