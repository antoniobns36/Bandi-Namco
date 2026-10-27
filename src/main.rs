
pub mod io {
    use std::io::{self, Write};

    pub fn ask_number() -> i32 {
        loop {
            print!("? ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
                
            match input.trim().parse::<i32>() {
                Ok(num) => return num,
                Err(_) => {
                    println!("Please enter a valid number.");
                }
            }
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Stop,
    Hunt,
    Continue,
}

pub fn turn_menu(fort_available: &mut bool, bulletss: i32) -> Action {
    loop {
         let choice = if *fort_available {
      
         println!("DO YOU WANT TO (1) HUNT, OR (2) CONTINUE");
         let input = crate::io::ask_number();

         if input == 1 {
             Action::Hunt
         } else {
             Action::Continue
         }
      } else {
 
        println!("DO YOU WANT TO (1) STOP AT THE NEXT FORT, (2) HUNT, OR ");
        println!("(3) CONTINUE");
        let input = crate::io::ask_number();

        match input {
            1 => Action::Stop,
            2 => Action::Hunt,
            _ => Action::Continue,
        }
   };

   if choice == Action::Hunt && bulletss < 40 {
       println!("TOUGH---YOU NEED MORE BULLETS TO GO HUNTING");

   } else {
    
       *fort_available = *fort_available;
       return choice;
       }
   } 
}

fn main() { 
     println!("Module compiled successfully");
}       

pub fn fort_purchases(
    cash: &mut f64,
    food: &mut f64,
    bullets: &mut i32,
    clothing: &mut f64,
    misc: &mut f64,
    miles: &mut f64,
){
   println!("ENTER WHAT YOU WISH TO SPEND ON THE FOLLOWING");
  
   
   let items = ["food", "ammunition", "clothing", "misc. supplies"];

   for item_name in items {
       print!("{} ", item_name);
       let amount = io::ask_number() as f64;

    
       if amount < 0.0 {
          continue;
       }

       if *cash - amount < 0.0 {
          println!("YOU DON'T HAVE THAT MUCH--KEEP YOUOR SPENDING DOWN");
          println!("YOU MISS YOUR CHANCE TO SPEND ON THAT ITEM");
          continue;
       }

  
       *cash -= amount;
  

       match item_name {
           "food" => {
               *food += amount * (2.0 / 3.0);
           }
           "ammunition" => {
               let gained_bullets = (amount * (50.0 * (2.0 / 3.0))) as i32;
               *bullets += gained_bullets;
           }
           "clothing" => {
               *clothing += amount * (2.0 / 3.0);
           }
           "misc. supplies" => {
               *misc += amount * (2.0 / 3.0);
           }
           _ => {}
        }
    }
   
    *miles -= 45.0;
}
