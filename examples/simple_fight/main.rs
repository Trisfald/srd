use srd::character::{class::FIGHTER, race::HILL_DWARF, Character};
use srd::rules::narrator::DebugNarrator;
use srd::util::seed_battle_prng;
use srd::{ability::*, init_srd_compendium, CreatureHandle, SRDRules};
use weasel::{Battle, BattleController, Server};

fn main() {
    // Initialize a compendium with the SRD rules.
    init_srd_compendium().unwrap();

    // Create the first character.
    let mut guy = Character::new("guy", HILL_DWARF, FIGHTER).unwrap();
    guy.add_ability(STRENGTH, AbilityScore::new(18).unwrap())
        .add_ability(DEXTERITY, AbilityScore::new(10).unwrap())
        .add_ability(CONSTITUTION, AbilityScore::new(10).unwrap());

    // Create the second character.
    let mut dude = Character::new("dude", HILL_DWARF, FIGHTER).unwrap();
    dude.add_ability(STRENGTH, AbilityScore::new(10).unwrap())
        .add_ability(DEXTERITY, AbilityScore::new(14).unwrap())
        .add_ability(CONSTITUTION, AbilityScore::new(14).unwrap());

    // Create a battle.
    let rules = SRDRules::new(std::sync::Arc::new(DebugNarrator::default()));
    let battle = Battle::builder(rules).build();
    let mut server = Server::builder(battle).build();

    // Seed the pseudo random number generator.
    seed_battle_prng(&mut server).unwrap();

    // Spawn both characters.
    let guy = guy.spawn(&mut server).unwrap();
    let dude = dude.spawn(&mut server).unwrap();

    // Print the characters' stats.
    print_character_stats(&guy);
    print_character_stats(&dude);

    // Fight until only one remains!
    while server.battle().entities().creatures().count() > 1 {
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

fn print_character_stats(_handle: &CreatureHandle) {
    // TODO print name
    // for ability in handle.abilities() {
    //     println!("{:?}", ability);
    // }
}
