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
// 9. DATA STRUCTURES & LOGIC
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CauseOfDeath {
    TrailTooLong,
    Snakebite,
    RiderMassacre,
    Starvation,
    UnaffordableDoctor,
    ExhaustedMedicalSupplies,
    WolfMassacre,
}

pub struct GameContext {
    pub miles: f64,
    pub miles_last_turn: f64,
    pub total_trip_mileage: f64,
    pub food: f64,
    pub bullets: i32,
    pub clothing: f64,
    pub misc_supplies: f64,
    pub cash: f64,
    pub oxen_fund: f64,
    pub skill_level: i32,
    pub eating_level: i32, // 1 = poorly, 2 = moderately, 3 = well
    pub turn_number: i32,
    pub injury_flag: bool,
    pub illness_flag: bool,
    pub blizzard_flag: bool,
    pub south_pass_cleared: bool,
    pub blue_mountains_cleared: bool,
}

pub fn roll_random_event(roll: u8) -> u8 {
    match roll {
        0..=5 => 1,
        6..=10 => 2,
        11..=12 => 3,
        13..=14 => 4,
        15..=16 => 5,
        17..=21 => 6,
        22..=31 => 7,
        32..=34 => 8,
        35..=36 => 9,
        37..=41 => 10,
        42..=43 => 11,
        44..=53 => 12,
        54..=63 => 13,
        64..=68 => 14,
        69..=94 => 15,
        _ => 16,
    }
}

pub fn apply_event(event_num: u8, ctx: &mut GameContext) -> Option<CauseOfDeath> {
    match event_num {
        1 => {
            println!("WAGON BREAKS DOWN--LOSE TIME AND SUPPLIES FIXING IT");
            let rand_miles = (rng::rnd() % 6) as f64; // up to 5 random miles
            ctx.miles -= 15.0 + rand_miles;
            ctx.misc_supplies -= 8.0;
        }
        2 => {
            println!("OX INJURES LEG---SLOWS YOU DOWN REST OF TRIP");
            ctx.miles -= 25.0;
            ctx.oxen_fund -= 20.0;
        }
        3 => {
            println!("BAD LUCK---YOUR DAUGHTER BROKE HER ARM YOU HAD TO STOP AND USE SUPPLIES TO MAKE A SLING");
            let rand_miles = (rng::rnd() % 5) as f64; // up to 4 random miles
            let rand_misc = (rng::rnd() % 4) as f64;  // up to 3 random misc
            ctx.miles -= 5.0 + rand_miles;
            ctx.misc_supplies -= 2.0 + rand_misc;
        }
        4 => {
            println!("OX WANDERS OFF---SPEND TIME LOOKING FOR IT");
            ctx.miles -= 17.0;
        }
        5 => {
            println!("YOUR SON GETS LOST---SPEND HALF THE DAY LOOKING FOR HIM");
            ctx.miles -= 10.0;
        }
        6 => {
            println!("UNSAFE WATER--LOSE TIME LOOKING FOR CLEAN SPRING");
            let rand_miles = (rng::rnd() % 11) as f64; // up to 10 random miles
            ctx.miles -= 2.0 + rand_miles;
        }
        7 => {
            if ctx.miles <= 950.0 {
                println!("HEAVY RAINS---TIME AND SUPPLIES LOST");
                ctx.food -= 10.0;
                ctx.bullets -= 500;
                ctx.misc_supplies -= 15.0;
                let rand_miles = (rng::rnd() % 11) as f64; // up to 10 random miles
                ctx.miles -= 5.0 + rand_miles;
            } else {
                let threshold = 22 + (rng::rnd() % 5); // 22-26 threshold
                if ctx.clothing >= threshold as f64 {
                    println!("COLD WEATHER---BRRRRRRR!--YOU HAVE ENOUGH CLOTHING TO KEEP YOU WARM");
                } else {
                    println!("COLD WEATHER---BRRRRRRR!--YOU DON'T HAVE ENOUGH CLOTHING TO KEEP YOU WARM");
                    if let Some(ending) = illness_check(ctx) {
                        return Some(ending);
                    }
                }
            }
        }
        8 => {
            println!("BANDITS ATTACK");
            if ctx.bullets < 0 {
                println!("YOU RAN OUT OF BULLETS---THEY GET LOTS OF CASH");
            }
        	let score = shooting_challenge(ctx.skill_level);
		ctx.bullets -= 20 * score;
		if score > 1 {
	 	     println!("YOU GOT SHOT IN THE LEG AND THEY TOOK ONE OF YOUR OXEN BETTER HAVE A DOC LOOK AT YOUR WOUND");
		     ctx.misc_supplies -= 5.0;
		     ctx.oxen_fund -= 20.0;
		} else {
		    println!("QUICKEST DRAW OUTSIDE OF DODGE CITY!!! YOU GOT 'EM!");
             }
        }
	9 => {
	   println!("THERE WAS A FIRE IN YOUR WAGON--FOOD AND SUPPLIES DAMAGE!");
	   ctx.food -= 40.0;
	   ctx.bullets -= 400;
	   let rand_misc = (rng::rnd() % 9) as f64;
	   ctx.misc_supplies -= 3.0 + rand_misc;
	   ctx.miles -= 15.0;
	}
	10 => {
	    println!("LOSE YOUR WAY IN HEAVY FOG---TIME IS LOST");
	    let rand_miles = (rng::rnd() % 6) as f64;
	    ctx.miles -= 10.0 + rand_miles;
	}
	11 => {
	    println!("YOU KILLED A POISONOUS SNAKE AFTER IT BIT YOU");
	    ctx.bullets -= 10;
	    ctx.misc_supplies -= 5.0;
	    if ctx.misc_supplies < 0.0 {
		println!("YOU DIE OF SNAKEBITE SINCE YOU HAVE NO MEDICINE");
		return Some(CauseOfDeath::Snakebite);
	    }
	}
	12 => {
	    println!("WAGON GETS SWAMPED FORDING RIVER--LOSE FOOD AND CLOTHES");
	    ctx.food -= 30.0;
	    ctx.clothing -= 20.0;
	    let rand_miles = (rng::rnd() % 21) as f64;
	    ctx.miles -= 20.0 + rand_miles;
	}
	13 => {
	    println!("WILD ANIMALS ATTACK!");
	    if ctx.bullets <= 39 {
	 	println!("YOU WERE TOO LOW ON BULLETS-- THE WOLVES OVERPOWERED YOU");
		ctx.injury_flag = true;
		return Some(CauseOfDeath::WolfMassacre);
	    }
	    let score = shooting_challenge(ctx.skill_level);
	    ctx.bullets -= 20 * score;
	    if score > 2 {
		println!("SLOW ON THE DRAW---THEY GOT AT YOUR FOOD AND CLOTHES");
		ctx.clothing -= 4.0 * (score as f64);
		ctx.food -= 8.0 * (score as f64);
	    } else {
		println!("NICE SHOOTIN' PARDNER---THEY DIDN'T GET MUCH");
	    }
	}
	14 => {
	    println!("HAIL STORM---SUPPLIES DAMAGED");
	    let rand_miles = (rng::rnd() % 11) as f64;
	    ctx.miles -= 5.0 + rand_miles;
	    ctx.bullets -= 200;
	    let rand_misc = (rng::rnd() % 4) as f64;
	    ctx.misc_supplies -= 4.0 + rand_misc;
	}
	15 => {
	    let roll = (rng::rnd() % 100) + 1;
	    let trigger = match ctx.eating_level {
		1 => true,
		2 => roll <= 75,
		_ => roll <= 50,
	     };
	if trigger {
	    if let Some(ending) = illness_check(ctx) {
	    	return Some(ending);
	    	}
	    }
	}
	16 => {
	    println!("HELPFUL INDIANS SHOW YOU WHERE TO FIND MORE FOOD");
	    ctx.food += 14.0;
	}
	_ => {}
    }
    None
}

pub fn cross_mountains(ctx: &mut GameContext) -> Option<CauseOfDeath> {
    if ctx.miles <= 950.0 {
    return None;
    }
    let rugged_roll = rng::rnd() % 100;
    let distance_from_peak = (ctx.miles - 1500.0).abs();
    let threshold = (50.0 - (distance_from_peak * 0.05)).max(10.0) as i32;
    if rugged_roll < threshold {
	println!("RUGGED MOUNTAINS");
	let outcome_roll = rng::rnd() % 100;
	if outcome_roll < 10 {
	   println!("YOU GOT LOST---LOSE VALUABLE TIME TRYING TO FIND TRAIL!");
	   ctx.miles -= 60.0;
	} else if outcome_roll < 20 {
	   println!("WAGON DAMAGED!---LOSE TIME AND SUPPLIES");
	   ctx.misc_supplies -= 5.0;
	   ctx.bullets -= 200;
	   let rand_miles = (rng::rnd() % 31) as f64;
	   ctx.miles -= 20.0 + rand_miles;
	} else {
	   let rand_miles = (rng::rnd() % 51) as f64;
	   ctx.miles -= 45.0 + rand_miles;
	   }
        }
    	if !ctx.south_pass_cleared {
            ctx.south_pass_cleared = true;
    	    let blizzard_roll = rng::rnd() % 100;
   	    if blizzard_roll < 80 {
    	    if let Some(ending) = trigger_blizzard(ctx) {
    	    return Some(ending);
    	    }
    	} else {
	   println!("YOU MADE IT SAFELY THROUGH SOUTH PASS--NO SNOW");
	   }
    }
    if ctx.miles >= 1700.0 && !ctx.blue_mountains_cleared {
    	ctx.blue_mountains_cleared = true;
    	let blizzard_roll = rng::rnd() % 100;
    	if blizzard_roll < 70 {
    	if let Some(ending) = trigger_blizzard(ctx) {
    	   return Some(ending);
    	   }
	}
    }
    None
}

fn trigger_blizzard(ctx: &mut GameContext) -> Option<CauseOfDeath> {
    println!("BLIZZARD IN MOUNTAIN PASS--TIME AND SUPPLIES LOST");
    ctx.blizzard_flag = true;
    ctx.food -= 25.0;
    ctx.misc_supplies -= 10.0;
    ctx.bullets -= 300;
    let rand_miles = (rng::rnd() % 41) as f64;
    ctx.miles -= 30.0 + rand_miles;
    let threshold = 18 + (rng::rnd() % 3);
    if ctx.clothing < threshold as f64 {
       if let Some(ending) = illness_check(ctx) {
       return Some(ending);
       }
    }
    None
}

pub fn illness_check(ctx: &mut GameContext) -> Option<CauseOfDeath> {
    let roll1 = (rng::rnd() % 100) + 1;
    let mild_chance = 10 + 35 * (ctx.eating_level - 1);
    if roll1 < mild_chance {
	println!("MILD ILLNESS---MEDICINE USED");
	ctx.miles -= 5.0;
	ctx.misc_supplies -= 2.0;
    } else {
	let roll2 = (rng::rnd() % 100) + 1;
	let denom = 4_i32.pow((ctx.eating_level - 1) as u32);
	let bad_chance = 100 - (40 / denom);
	if roll2 < bad_chance {
	   println!("BAD ILLNESS---MEDICINE USED");
	   ctx.miles -= 5.0;
	   ctx.misc_supplies -= 2.0;
    } else {
	println!("SERIOUS ILLNESS---\nYOU MUST STOP FOR MEDICAL ATTENTION");
	ctx.misc_supplies -= 10.0;
	ctx.illness_flag = true;
        }
    }
    if ctx.misc_supplies < 0.0 {
    return Some(CauseOfDeath::ExhaustedMedicalSupplies);
    }
    None
}

pub fn resolve_ending(reason: CauseOfDeath, ctx: &mut GameContext) {
    match reason {
    CauseOfDeath::TrailTooLong => {
	println!("YOU HAVE BEEN ON THE TRAIL TOO LONG ------ YOUR FAMILY DIES IN THE FIRST BLIZZARD OF WINTER");
    }
    CauseOfDeath::Snakebite => {
        println!("YOU DIE OF SNAKEBITE SINCE YOU HAVE NO MEDICINE");
    }
    CauseOfDeath::RiderMassacre => {
        println!("YOU RUN OUT OF BULLETS AND GOT MASSACRED BY THE RIDERS");
    }
    CauseOfDeath::Starvation => {
        println!("YOU RUN OUT OF FOOD AND STARVED TO DEATH");
    }
    CauseOfDeath::UnaffordableDoctor => {
    ctx.cash = 0.0;
        println!("YOU CAN'T AFFORD A DOCTOR");
        print!("YOU DIED OF ");
        if ctx.injury_flag { println!("INJURIES"); } else { println!("PNEUMONIA"); }
    }
    CauseOfDeath::ExhaustedMedicalSupplies => {
        println!("YOU RAN OUT OF MEDICAL SUPPLIES");
        println!("YOU DIED OF PNEUMONIA");
    }
    CauseOfDeath::WolfMassacre => {
        print!("YOU DIED OF ");
        if ctx.injury_flag { println!("INJURIES"); } else { println!("PNEUMONIA"); }
    }
}
println!();
println!("DUE TO YOUR UNFORTUNATE SITUATION, THERE ARE A FEW FORMALITIES WE MUST GO THROUGH");
io::ask("WOULD YOU LIKE A MINISTER? ");
io::ask("WOULD YOU LIKE A FANCY FUNERAL? ");
let kin = io::ask("WOULD YOU LIKE US TO INFORM YOUR NEXT OF KIN? ");
if kin.trim().to_uppercase() == "YES" {
   println!("THAT WILL BE $50 FOR THE TELEGRAPH CHARGE.");
} else {
   println!("BUT YOUR AUNT SADIE IN ST. LOUIS IS REALLY WORRIED ABOUT YOU");
}
println!("WE THANK YOU FOR THIS INFORMATION AND WE ARE SORRY YOU DIDN'T MAKE IT TO THE GREAT TERRITORY OF OREGON");
println!("BETTER LUCK NEXT TIME");
println!("SINCERELY");
println!("THE OREGON CITY CHAMBER OF COMMERCE");
}
pub fn arrive_at_oregon_city(ctx: &mut GameContext) {
    let fraction_traveled = (2040.0 - ctx.miles_last_turn) / (ctx.total_trip_mileage - ctx.miles_last_turn);
    ctx.food += (1.0 - fraction_traveled) * (8.0 + 5.0 * (ctx.eating_level as f64));
    println!("YOU FINALLY ARRIVED AT OREGON CITY AFTER 2040 LONG MILES---HOORAY!!!!! A REAL PIONEER!");
    let mut days = fraction_traveled * 14.0;
    let turn_calc = (ctx.turn_number * 14) as f64 + days;
    days += 1.0;
    let weekday_idx = if days >= 5.0 { (days - 7.0) as usize } else { days as usize };
    let weekdays = ["MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY", "SUNDAY"];
    let day_string = weekdays[weekday_idx % 7];
    print!("{} ", day_string);
    let t_num = turn_calc as i32;
    if t_num <= 124 {
    println!("JULY {} 1847", t_num - 93);
    } else if t_num <= 155 {
    println!("AUGUST {} 1847", t_num - 124);
    } else if t_num <= 185 {
    println!("SEPTEMBER {} 1847", t_num - 155);
    } else if t_num <= 216 {
    println!("OCTOBER {} 1847", t_num - 185);
    } else if t_num <= 246 {
    println!("NOVEMBER {} 216", t_num - 216);
    } else {
    println!("DECEMBER {} 1847", t_num - 246);
    }
    let disp_food = ctx.food.max(0.0) as i32;
    let disp_bullets = ctx.bullets.max(0);
    let disp_clothing = ctx.clothing.max(0.0) as i32;
    let disp_misc = ctx.misc_supplies.max(1.0) as i32;
    let disp_cash = ctx.cash.max(0.0) as i32;
    println!("\n{:<10} {:<10} {:<10} {:<12} {:<10}", "FOOD", "BULLETS", "CLOTHING", "MISC. SUPP.", "CASH");
    println!("{:<10} {:<10} {:<10} {:<12} ${:<10}", disp_food, disp_bullets, disp_clothing, disp_misc, disp_cash);
    println!();
    println!("PRESIDENT JAMES K. POLK SENDS YOU HIS HEARTIEST CONGRATULATIONS AND WISHES YOU A PROSPEROUS LIFE AHEAD AT YOUR NEW HOME");
    } 
fn main() {
    println!("All game systems compiled successfully!");
}
