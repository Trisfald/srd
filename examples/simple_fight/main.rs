use srd::ability::*;
use srd::character::{class::FIGHTER, race::HILL_DWARF, Character};
use srd::init_srd_compendium;
use weasel::{Battle, Server};

fn main() {
    // Initialize a compendium with the SRD rules.
    init_srd_compendium();

    // Create the first character.
    let mut guy = Character::new("guy", HILL_DWARF, FIGHTER);
    guy.add_ability(STRENGTH, AbilityScore::new(18).unwrap())
        .add_ability(DEXTERITY, AbilityScore::new(10).unwrap())
        .add_ability(CONSTITUTION, AbilityScore::new(10).unwrap());

    // Create the second character.
    let mut dude = Character::new("dude", HILL_DWARF, FIGHTER);
    dude.add_ability(STRENGTH, AbilityScore::new(10).unwrap())
        .add_ability(DEXTERITY, AbilityScore::new(14).unwrap())
        .add_ability(CONSTITUTION, AbilityScore::new(14).unwrap());

    // Create a battle.
    // let rules = SRDRules::new();
    // let battle = Battle::builder(rules).build();
    // let mut server = Server::builder(battle).build();

    // Seed the pseudo random number generator.
    seed_battle_prng(&mut server).unwrap();

    // Spawn both characters.
    // let guy = guy.spawn();
    // let dude = dude.spawn();

    // Print the characters' stats.
    // for ability in guy.abilities() {
    //     println!("{:?}", ability);
    // }
    // for ability in dude.abilities() {
    //     println!("{:?}", ability);
    // }

    // Fight until only one remains!
    while true {
        // Get who acts now.
        // let id = xxx.next_actor();

        // get all possible actions and print..
        // let handle = CharacterHandle::new(id, &mut server);
        // TODO

        // maybe move? or just spawn the two close to each other..

        // attack..
        // handle.attack(...);
    }
}
