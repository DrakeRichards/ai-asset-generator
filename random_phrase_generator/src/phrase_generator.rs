#![allow(dead_code)]

use crate::weighted_items::WeightedItemList;
use anyhow::Result;
use ex::fs;
use std::path::PathBuf;

/// A combination of several related weighted item lists. Used to generate random phrases.
/// Generally should be created from CSV files or strings using the `from_csv_files` or `from_csv_strings` methods.
/// Once created, random phrases can be generated using the `generate_random_phrase` method.
pub struct RandomphraseGenerator {
    /// The weighted item lists.
    weighted_item_lists: Vec<WeightedItemList>,
}

impl RandomphraseGenerator {
    /// Create a new RandomphraseGenerator from a list of CSV files.
    pub fn from_csv_files(csv_files: &Vec<PathBuf>) -> Result<RandomphraseGenerator> {
        let mut weighted_item_lists = Vec::new();
        for csv_file in csv_files {
            let csv = fs::read_to_string(csv_file)?;
            weighted_item_lists.push(WeightedItemList::from_csv(&csv)?);
        }
        Ok(RandomphraseGenerator {
            weighted_item_lists,
        })
    }

    /// Create a new RandomphraseGenerator from a list of CSV strings.
    pub fn from_csv_strings(csv_strings: Vec<&str>) -> Result<RandomphraseGenerator> {
        let mut weighted_item_lists = Vec::new();
        for csv_string in csv_strings {
            weighted_item_lists.push(WeightedItemList::from_csv(csv_string)?);
        }
        Ok(RandomphraseGenerator {
            weighted_item_lists,
        })
    }

    /// Generate a random phrase from the weighted item lists.
    pub fn generate_random_phrase(&self) -> String {
        self.weighted_item_lists
            .iter()
            .map(|weighted_item_list| weighted_item_list.pick_random())
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::Write,
        path::{Path, PathBuf},
    };

    #[test]
    fn test_from_csv_files() -> Result<()> {
        /// Generate CSV files for testing.
        fn generate_test_csv_files() -> Result<Vec<PathBuf>> {
            struct CsvFile {
                text: &'static str,
                path: PathBuf,
            }

            let csv_files = vec![
                CsvFile {
                    text: "value,weight\nred,1\nblue,1\nyellow,1",
                    path: Path::new("colors.csv").to_path_buf(),
                },
                CsvFile {
                    text: "value,weight\nNew York,1\nLos Angeles,1\nChicago,1",
                    path: Path::new("cities.csv").to_path_buf(),
                },
            ];

            let mut paths = Vec::new();
            for csv_file in csv_files {
                let mut file = File::create(&csv_file.path)?;
                file.write_all(csv_file.text.as_bytes())?;
                paths.push(csv_file.path);
            }
            Ok(paths)
        }

        let csv_files = generate_test_csv_files()?;
        let phrase_generator = RandomphraseGenerator::from_csv_files(&csv_files)?;
        // The phrase generator should have two weighted item lists.
        assert_eq!(phrase_generator.weighted_item_lists.len(), 2);
        let random_phrase = phrase_generator.generate_random_phrase();
        // The random phrase should have at least one space.
        assert!(random_phrase.contains(" "));
        // The phrase should have at least two characters.
        assert!(random_phrase.len() >= 2);
        // Clean up the CSV files.
        for csv_file in csv_files {
            fs::remove_file(csv_file)?;
        }
        Ok(())
    }

    #[test]
    fn test_from_csv_strings() -> Result<()> {
        let colors_csv = "value,weight\nred,1\nblue,1\nyellow,1";
        let cities_csv = "value,weight\nNew York,1\nLos Angeles,1\nChicago,1";
        let csv_strings = vec![colors_csv, cities_csv];
        let phrase_generator = RandomphraseGenerator::from_csv_strings(csv_strings)?;
        // The phrase generator should have two weighted item lists.
        assert_eq!(phrase_generator.weighted_item_lists.len(), 2);
        let random_phrase = phrase_generator.generate_random_phrase();
        // The random phrase should have at least one space.
        assert!(random_phrase.contains(" "));
        // The phrase should have at least two characters.
        assert!(random_phrase.len() >= 2);
        Ok(())
    }
}
