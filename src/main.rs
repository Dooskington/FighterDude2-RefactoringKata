/*
    Fighting Game Refactoring Kata
    This program was designed years ago to help calculate player and NPC power levels for the new hit video game, FIGHTER DUDE 2.
    It's your first day on the job and you're the new maintainer for the tool, but you don't understand how it works!
    Luckily, the original Author at least left a few tests and comments behind. Maybe some refactoring is in order...

    It's important that no behavior changes as a result of these refactorings!

    [ ] slide statement
    [ ] extract function
    [ ] inline function
    [ ] inline variable
    [ ] introduce parameter object
    [ ] split variable
    [ ] replace temp with query
    [ ] change function declaration
        [ ] simple mechanics
        [ ] migration mechanics
    [ ] write an actual good test for calculation?
*/

mod tests;

fn main() {
    println!("Hello, world!");

    // gather and display my power level
    let gear_score = 123;
    let fighting_skill_level = 15;
    let happiness = 1000;
    let power_level = calculation(gear_score, fighting_skill_level, happiness).expect("Failed to calculate!");
    println!("My power level is {}!", power_level);

    // ... fight some monsters

    // you gained experience points. fighting skill level has changed
    let fighting_skill_level = fighting_skill_level + skill_level_increase_from_exp_points(exp_from_enemy(get_power_of_sun()));
    let power_level = calculation(gear_score, fighting_skill_level, happiness).expect("Failed to calculate!");
    println!("My new power level is {}!", power_level);

    let enemy_power_level = calculation(5, 10, 1).expect("Failed to calculate!");
    println!("The enemy power level is {}!", enemy_power_level);
}

// get the power of the sun
pub fn get_power_of_sun() -> i32 {
    5
}

// enemies give more exp if the sun is out more??
pub fn exp_from_enemy(sun_power: i32) -> i32 {
    500 + (sun_power * 3)
}

// calculate your total power level, based on your gear score, fighting skill level, your happiness, and the power of the sun and moon
pub fn calculation(gear_score: i32, fighting_skill_level: i32, happiness: i32) -> Option<i32> {
    if (gear_score < 0) || (fighting_skill_level < 0) {
        return None;
    }

    if happiness < 0 {
        // being unhappy makes you evil and thus the most powerful of all
        return Some(i32::MAX);
    }

    // calculate fighting power based on gear score
    let mut fighting_power = 0;
    for i in 0..gear_score {
        fighting_power += (fighting_skill_level * 2) + i;
    }

    // true happiness actually comes from your fighting power.
    let happiness = happiness + (fighting_power / 3); 

    let moon_power = 5 * 2;
    let sun_power = get_power_of_sun() + happiness;

    Some(fighting_power + moon_power + sun_power)
}

pub fn skill_level_increase_from_exp_points(exp: i32) -> i32 {
    exp + ((exp * exp) as f32 * 0.1).floor() as i32
}
