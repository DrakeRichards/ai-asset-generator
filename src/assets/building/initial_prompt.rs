use crate::weighted_random::WeightedItemList;
use anyhow::Result;

const ADJECTIVES_FILE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/rpg-generation-assets/buildings/adjectives.csv"
));
const TYPES_FILE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/rpg-generation-assets/buildings/types.csv"
));

pub fn generate_initial_prompt() -> Result<String> {
    let adjectives: WeightedItemList = WeightedItemList::from_csv(ADJECTIVES_FILE)?;
    let building_types: WeightedItemList = WeightedItemList::from_csv(TYPES_FILE)?;

    let adjective: &str = adjectives.pick_random();
    let building_type: &str = building_types.pick_random();

    Ok(format!("{adjective} {building_type}"))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    use super::*;

    #[test]
    fn test_generate_initial_prompt() {
        let prompt: String = generate_initial_prompt().unwrap();
        println!("{}", prompt);
        assert!(!prompt.is_empty());
    }
}
