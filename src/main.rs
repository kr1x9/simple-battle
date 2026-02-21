use std::io;
use rand::RngExt;

fn main() {
    // TODO: Ask the player to enter their character's name and store it in a variable

    // TODO: Ask the player to enter the enemy's name and store it in a variable

    // TODO: Declare mutable variables for player_health and enemy_health, both set to 100.0

    println!("A battle begins between {} and {}!", player_name, enemy_name);

    loop {
        // TODO: Generate a random number between 1 and 100 and store it in player_roll
        // Then calculate the player's damage by subtracting the result of attack(enemy_health, player_roll)
        // from the current enemy_health. Store this in player_damage.
        // Then subtract player_damage from enemy_health to get the new enemy_health.

        // TODO: Do the same for the enemy attacking the player.
        // Generate enemy_roll, calculate enemy_damage and subtract it from player_health.

        // TODO: Print the roll percentage, damage dealt, and current health of both characters

        // TODO: Check if both health values have dropped below 1 at the same time.
        // If so, declare a draw and break out of the loop.
        // Otherwise check if either one has dropped below 1 individually,
        // declare that character the victor and break out of the loop.
        // Hint: check for the draw condition first
    }
}

// TODO: Write a function called attack that takes a health value and a damage_roll as parameters.
// The damage_roll is a whole number between 1 and 100.
// To turn that whole number into a percentage, divide it by 100.0
// For example: a roll of 33 becomes 33 / 100.0 = 0.33
// Multiply the health value by that number to find out how much health is lost,
// then subtract it from the current health to get the new health value.
// Return the new health value.
//
// Example:
//   Current health is 100, damage_roll is 33
//   33 / 100.0 = 0.33
//   100 * 0.33 = 33 damage dealt
//   100 - 33 = 67 new health
//
//   Next round health is 67, damage_roll is 50
//   50 / 100.0 = 0.50
//   67 * 0.50 = 33.5 damage dealt
//   67 - 33.5 = 33.5 new health
//
// Hint: think carefully about what data type can hold a number with a decimal point
