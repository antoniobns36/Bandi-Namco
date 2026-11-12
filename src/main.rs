use std::time::Instant;

// ==========================================
// 1. IO MODULE
// ==========================================
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

    pub fn ask(prompt: &str) -> String {
        if !prompt.is_empty() {
            print!("{}", prompt);
        } else {
            print!("? ");
        }
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}

pub fn get_shooting_skill_level() -> i32 {
	println!();
	println!("HOW GOOD A SHOT ARE YOU WITH YOUR RIFLE?");
	println!("(1) ACE MARKSMAN, (2) GOOD SHOT, (3) fAIR TO MIDDLIN'");
	println!("(4) NEED MORE PRACTICE, (5) SHAKY KNEES");
	println!("ENTER ONE OF THE ABOVE -- THE BETTER YOU CLAIM YOU ARE, THE");
	println!("FASTER YOU'LL HAVE TO BE WITH YOUR GUN TO BE SUCCESSFUL.");
	let skill_level = io::ask_number();
	print!("{}", skill_level);
	println!("");
	return skill_level
	
}

// ==========================================
// 10. PURCHASE SUPPLIES
// ==========================================
pub struct Supplies {
	pub oxen: i32,
	pub food: i32,
	pub bullets: i32,
	pub clothing: i32,
	pub misc: i32,
	pub cash: i32 
}
	
pub fn purchase_supplies() -> Supplies {
	let budget = 700;
	let (oxen, food, bullets, clothing, misc, cash) = loop {
		let oxen = loop {
				println!("HOW MUCH DO YOU WANT TO SEPND ON YOUR OXEN TEAM?");
				let oxen = io::ask_number();
				if oxen < 200 {
					println!("NOT ENOUGH")
				} else if oxen > 300 {
					println!("TOO MUCH")
				} else {
					print!(" {}", oxen);
					break oxen
				}
		};
		
		let food = loop {
			println!("HOW MUCH DO YOU WANT TO SPEND ON FOOD");
			let food = io::ask_number();
			print!("{}", food);
			
			if food < 0 {
				println!("REDO FROM START")
			} else {
				break food
				}
			};
			
		let ammunition = loop {
			println!("HOW MUCH DO YOU WANT TO SPEND ON AMMUNITION");
			let ammunition = io::ask_number();
			print!("{}", ammunition);
			if ammunition < 0 {
				println!("REDO FROM START")
			} else {
				break ammunition
				}
			};
		
		let clothing = loop {
			println!("HOW MUCH DO YOU WANT TO SPEND ON CLOTHING");
			let clothing = io::ask_number();
			print!("{}", clothing);
			if clothing < 0 {
				println!("REDO FROM START")
			} else {
				break clothing
				}
			};

		let misc = loop {			
			println!("HOW MUCH DO YOU WANT TO SPEND ON MISCELLANEOUS SUPPLIES");
			let misc = io::ask_number();
			print!("{}", misc);
			if misc < 0 {
				println!("REDO FROM START")
			} else {
				break misc
				}
			};

		let bullets = ammunition * 50;
		let total = oxen + food + ammunition + clothing + misc;

		if total > budget {
			println!("YOU OVERSPENT--YOU ONLY HAD $700 TO SPEND. BUY AGAIN")
		} else {
			let cash = budget - total;
			println!("AFTER ALL YOUR PURCHASES , YOU NOW HAVE {} DOLLARS LEFT", cash);
			break (oxen, food, bullets, clothing, misc, cash)
		}
	};

	println!();

	println!("MONDAY MARCH 29 1847");

	println!();

	println!("TOTAL MILEAGE IS 0");
	println!("FOOD     BULLETS     CLOTHING     MISC. SUPP.     CASH");
	println!("{}      {}         {}         {}               {}", food, bullets, clothing, misc, cash);

	return Supplies {
		oxen,
		food,
		bullets,
		clothing,
		misc,
		cash
		}
}

const DATES: [&str; 19] = [
	"APRIL 12", "APRIL 26", "MAY 10", "MAY 24", "JUNE 7", "JUNE 21", "JULY 5", "JULY 19", "AUGUST 2", 
	"AUGUST 16", "AUGUST 30", "SEPTEMBER 13", "SEPTEMBER 27", "OCTOBER 11", "OCTOBER 25", "NOVEMBER 8", "NOVEMBER 22", "DECEMBER 6", "DECEMBER 20"
	];
 
pub fn advance_turn_date() {
	let mut date = 0;
	if date > DATES.len() {
		println!();
		println!("YOU HAVE BEEN ON THE TRAIL TOO LONG");
		println!("YOUR FAMILY DIES IN THE FIRST BLIZZARD OF WINTER");
		println!()
	} else {
		println!();
		format!("MONDAY {} 1847", DATES[date]);
		println!();
		date += 1
	}
}

pub fn print_status_table(supplies: &mut Supplies, miles: &mut _mileage) {
	if supplies.food < 13 {
		println!("YOU'D BEETER DO SOME HUNTING OR BUY FOOD SOON!!!!");
		}
	if supplies.food < 0 {
		supplies.food = 0
		};
	if supplies.bullets < 0 {
		supplies.bullets = 0
		};
	if supplies.clothing < 0 {
		supplies.clothing = 0
		};
	if supplies.misc < 0 {
		supplies.misc = 0
		};
	println!("TOTAL MILEAGE IS {}", miles);
	println!();
	println!("FOOD     BULLETS     CLOTHING     MISC. SUPP.     CASH");
	println!("{}      {}         {}         {}               {}", supplies.food, supplies.bullets, supplies.clothing, supplies.misc, supplies.cash)
}

pub fn apply_doctor_bill(supplies: &mut Supplies, ill: &mut illness, injured: &mut injury) -> bool {
	if !*ill | !*injured {
		return true
	}; 
	
	if supplies.cash >= 20 {
		ill = false;
		injured = false;
		print!("DOCTOR'S BILL IS $20");
		supplies.cash -= 20;
		return true	 
	}
	else {
		println!("YOU CAN'T AFFORD A DOCTOR");
			return false
	}
}


fn main() {
    println!("All game systems compiled successfully!");
}
