use std::time::Instant;

// ==========================================
// 1. IO MODULE
// ==========================================
pub mod io {.
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

// ==========================================
// 2. RNG MODULE
// ==========================================
pub mod rng {
    use std::cell::Cell;
    thread_local! {
        static SEED: Cell<u32> = Cell::new(12345);
    }
    pub fn rnd() -> i32 {
        SEED.with(|seed| {
            let mut val = seed.get();
            val = val.wrapping_mul(1103515245).wrapping_add(12345);
            seed.set(val);
            (val % 1000) as i32
        })
    }
}

// ==========================================
// 3. ACTION ENUM
// ==========================================
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Stop,
    Hunt,
    Continue,
}

// ==========================================
// 4. SHOOTING CHALLENGE
// ==========================================
pub fn shooting_challenge(skill_level: i32) -> i32 {
    let words = ["BANG", "BLAM", "POW", "WHAM"];
    
    let index = (rng::rnd() as usize) % words.len();
    let target_word = words[index];

    println!("TYPE\n{}", target_word);

    let start_time = Instant::now();
    let player_input = io::ask("");
    let elapsed = start_time.elapsed().as_secs_f64();

    let skill_factor = if skill_level > 0 { skill_level as f64 } else { 1.0 };
    let mut score = (elapsed * 10.0 / skill_factor).round() as i32;
    if score < 1 { score = 1; }

    if player_input.trim() != target_word {
        score = 9;
    }

    score
}

// ==========================================
// 5. RESOLVE HUNT OUTCOME
// ==========================================
pub fn resolve_hunt_outcome(score: i32, bullets: &mut i32, food: &mut f64) {
    if score <= 1 {
        println!("RIGHT BETWEEN THE EYES---YOU GOT A BIG ONE!!!!");
        println!("FULL BELLIES TONIGHT!");

        let food_bonus = (rng::rnd() % 7) as f64;
        *food += 52.0 + food_bonus;

        let bullet_cost = 10 + ((rng::rnd() % 5) as i32);
        *bullets -= bullet_cost;
    } else {
        let miss_chance = (score as f64) * 0.13;
        let hit = (rng::rnd() % 100) as f64 >= (miss_chance * 100.0);

        if !hit || score >= 9 {
            println!("YOU MISSED---AND YOUR DINNER GOT AWAY.....");
        } else {
            println!("NICE SHOT--RIGHT ON TARGET--GOOD EATIN' TONIGHT!!");

            let food_earned = (48 - 2 * score) as f64;
            if food_earned > 0.0 {
                *food += food_earned;
            }

            let bullets_spent = 10 + 3 * score;
            *bullets -= bullets_spent;
        }
    }
}

// ==========================================
// 6. GO HUNTING
// ==========================================
pub fn go_hunting(bullets: &mut i32, food: &mut f64, miles: &mut f64, skill_level: i32) {
    if *bullets < 40 {
        println!("TOUGH---YOU NEED MORE BULLETS TO GO HUNTING");
        return;
    }

    *miles -= 45.0;
    let score = shooting_challenge(skill_level);
    resolve_hunt_outcome(score, bullets, food);
}

// ==========================================
// 7. FORT PURCHASES
// ==========================================
pub fn fort_purchases(
    cash: &mut f64,
    food: &mut f64,
    bullets: &mut i32,
    clothing: &mut f64,
    misc: &mut f64,
    miles: &mut f64,
) {
    println!("ENTER WHAT YOU WISH TO SPEND ON THE FOLLOWING");

    let items = ["food", "ammunition", "clothing", "misc. supplies"];

    for item_name in items {
        print!("{} ", item_name);
        let amount = io::ask_number() as f64;

        if amount < 0.0 {
            continue;
        }

        if *cash - amount < 0.0 {
            println!("YOU DON'T HAVE THAT MUCH--KEEP YOUR SPENDING DOWN");
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

// ==========================================
// 8. TURN MENU
// ==========================================
pub fn turn_menu(fort_available: &mut bool, bullets: i32) -> Action {
    loop {
        let choice = if !*fort_available {
            println!("DO YOU WANT TO (1) HUNT, OR (2) CONTINUE");
            let input = io::ask_number();
            
            if input == 1 {
                Action::Hunt
            } else {
                Action::Continue
            }
        } else {
            print!("DO YOU WANT TO (1) STOP AT THE NEXT FORT, (2) HUNT, OR ");
            println!("(3) CONTINUE");
            let input = io::ask_number();
            
            match input {
                1 => Action::Stop,
                2 => Action::Hunt,
                _ => Action::Continue,
            }
        };

        if choice == Action::Hunt && bullets < 40 {
            println!("TOUGH---YOU NEED MORE BULLETS TO GO HUNTING");
        } else {
            *fort_available = !*fort_available;
            return choice;
        }
    }
}
// ==========================================
// 9. GET SHOOTIN SKILL LEVEL
// ==========================================
pub fn get_shooting_skill_level() -> i32 {
	println!();
	println!("HOW GOOD A SHOT ARE YOU WITH YOUR RIFLE?");
	println!("(1) ACE MARKSMAN, (2) GOOD SHOT, (3) fAIR TO MIDDLIN'");
	println!("(4) NEED MORE PRACTICE, (5) SHAKY KNEES");
	println!("ENTER ONE OF THE ABOVE -- THE BETTER YOU CLAIM YOU ARE, THE");
	println!("FASTER YOU'LL HAVE TO BE WITH YOUR GUN TO BE SUCCESSFUL.");
	let skill_level = io::ask_number():
	print!(skill_level):
	println!("");
	return skill_level
	
}

// ==========================================
// 10. PURCHASE SUPPLIES
// ==========================================
struct Supplies {
	oxen: i32,
	food: i32,
	bullets: i32,
	clothing: i32,
	misc: i32,
	cash: i32 
}
	
pub fn purchase_supplies() -> Supplies {
	let budget = 700
	loop {
		loop {
			let oxen = io::ask_number("HOW MUCH DO YOU WANT TO SEPND ON YOUR OXEN TEAM?");
			if oxen < 200 {
				println!("NOT ENOUGH")
			} else if oxen > 300 {
				prinln!("TOO MUCH")
			} else {
				print!(" {}", oxen);
				break
			}
		};
		
		loop {
			println!("HOW MUCH DO YOU WANT TO SPEND ON FOOD");
			let food = io::ask_number();
			print!(food);
			if food < 0 {
				println!("REDO FROM START")
			} else {
				break
			}
		};
			
		loop {
			println!("HOW MUCH DO YOU WANT TO SPEND ON AMMUNITION");
			let ammunition = io::ask_number();
			print!(ammunition);
			if ammunition < 0 {
				println!("REDO FROM START")
			} else {
				break
			}
		};
		
		loop {
			println!("HOW MUCH DO YOU WANT TO SPEND ON CLOTHING");
			let clothing = io::ask_number();
			print!(clothing);
			if clothing < 0 {
				println!("REDO FROM START")
			} else {
				break
			}
		};

		loop {			
			println!("HOW MUCH DO YOU WANT TO SPEND ON MISCELLANEOUS SUPPLIES");
			let misc = io::ask_number();
			print!(misc);
			if misc < 0 {
				println!("REDO FROM START")
			} else {
				break
			}
		};

		let bullets = ammunition * 50;
		let total = oxen + food + ammunition + clothing + misc;

		if total > budget {
			println!("YOU OVERSPENT--YOU ONLY HAD $700 TO SPEND. BUY AGAIN")
		} else {
			let cash = budget - total
			println!("AFTER ALL YOUR PURCHASES , YOU NOW HAVE {} DOLLARS LEFT", cash);
			break			
		}
	};

	println!();

	println!("MONDAY MARCH 29 1847");

	println!();

	println!("TOTAL MILEAGE IS 0");
	println!("FOOD     BULLETS     CLOTHING     MISC. SUPP.     CASH");
	println!("{}      {}         {}         {}               {}", food, bullets, clothing, misc, cash)

	return Supplies {
		oxen,
		food,
		bullets,
		clothing,
		misc,
		cash
		}
}

const DATES: [&str, 19] = [
	"APRIL 12", "APRIL 26", "MAY 10", "MAY 24", "JUNE 7", "JUNE 21", "JULY 5", "JULY 19", "AUGUST 2", 
	"AUGUST 16", "AUGUST 30", "SEPTEMBER 13", "SEPTEMBER 27", "OCTOBER 11", "OCTOBER 25", "NOVEMBER 8", "NOVEMBER 22", "DECEMBER 6", "DECEMBER 20"
	];
 
pub fn advance_turn_date() -> String {
	let date = 0;
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

pub fn print_status_table(supplies: &mut Supplies, miles: &mut mileage) ->  {
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
	println!()
	println!("FOOD     BULLETS     CLOTHING     MISC. SUPP.     CASH");
	println!("{}      {}         {}         {}               {}", food, bullets, clothing, misc, cash)
}

pub fn apply_doctor_bill() -> {
	if *flag && cash >= 20 {
		print!("DOCTOR'S BILL IS $20");
		cash -= 20;
		flag = false
	} else {
		println!("YOU CAN'T AFFORD A DOCTOR");
		end game 		 

fn main() {
    println!("All game systems compiled successfully!");
}
