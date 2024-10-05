use super::weighted_random::WeightedItemList;

const GENDERS_FILE: &str = include_str!("../../../rpg-generation-assets/characters/genders.csv");
const RACES_FILE: &str = include_str!("../../../rpg-generation-assets/characters/races.csv");
const OCCUPATIONS_FILE: &str =
    include_str!("../../../rpg-generation-assets/characters/occupations.csv");

/// Generate a semi-random initial prompt for generating a character based on values provided in CSV files.
pub fn generate_initial_prompt() -> Result<String, std::io::Error> {
    let genders: WeightedItemList = WeightedItemList::from_csv(GENDERS_FILE)?;
    let races: WeightedItemList = WeightedItemList::from_csv(RACES_FILE)?;
    let occupations: WeightedItemList = WeightedItemList::from_csv(OCCUPATIONS_FILE)?;

    let gender: &str = genders.pick_random();
    let race: &str = races.pick_random();
    let occupation: &str = occupations.pick_random();

    Ok(format!("{gender} {race} {occupation}"))
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
