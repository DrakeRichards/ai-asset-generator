use super::weighted_random::WeightedItemList;

const ADJECTIVES_FILE: &str = include_str!("../../rpg-generation-assets/buildings/adjectives.csv");
const TYPES_FILE: &str = include_str!("../../rpg-generation-assets/buildings/types.csv");

pub fn generate_initial_prompt() -> String {
    let adjectives: WeightedItemList =
        WeightedItemList::from_csv(ADJECTIVES_FILE).expect("Failed to generate adjectives list.");
    let building_types: WeightedItemList =
        WeightedItemList::from_csv(TYPES_FILE).expect("Failed to generate building types list.");

    let adjective: &str = adjectives.pick_random();
    let building_type: &str = building_types.pick_random();

    format!("{adjective} {building_type}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_initial_prompt() {
        let prompt: String = generate_initial_prompt();
        println!("{}", prompt);
        assert!(!prompt.is_empty());
    }
}
