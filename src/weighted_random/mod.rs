//! A weighted random item picker.

use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::Deserialize;
use std::io::{Error, ErrorKind};

/// A weighted item.
#[derive(Debug, Deserialize)]
struct WeightedItem {
    /// The value of the item.
    value: String,
    /// The weight of the item.
    weight: u32,
}

/// A vector of weighted items.
pub struct WeightedItemList {
    /// The values in the list.
    pub values: Vec<String>,
    /// The weighted index of the list.
    pub weighted_index: WeightedIndex<u32>,
}

/// Pick a random item from a list of weighted items.
impl WeightedItemList {
    /// Pick a random item from a list of weighted items.
    pub fn pick_random(&self) -> &str {
        &self.values[self.weighted_index.sample(&mut rand::thread_rng())]
    }

    /// Convert the values in a .csv file to a Vec of WeightedItem structs.
    pub fn from_csv(csv: &str) -> Result<WeightedItemList, Error> {
        // Check that the CSV file is not empty.
        if csv.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "The CSV file is empty."));
        }

        // Build the csv reader
        let mut csv_reader = csv::ReaderBuilder::new().from_reader(csv.as_bytes());

        // Check that the CSV has two headers: value and weight.
        let headers = csv_reader.headers()?;
        if headers.len() != 2 || headers[0] != *"value" || headers[1] != *"weight" {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "The CSV file must have two headers: value and weight.",
            ));
        }

        // Build a vector of WeightedItem structs from the CSV file.
        let items = csv_reader
            .deserialize()
            .map(|item| item.unwrap())
            .collect::<Vec<WeightedItem>>();
        let values: Vec<String> = items.iter().map(|item| item.value.clone()).collect();
        let weights: Vec<u32> = items.iter().map(|item| item.weight).collect();

        // Create a WeightedIndex from the weights vector.
        let weighted_index: WeightedIndex<u32> = WeightedIndex::new(weights).unwrap();

        Ok(WeightedItemList {
            values,
            weighted_index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pick_random() {
        let values: Vec<String> = ["a", "b", "c", "d", "e"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let weights = vec![1, 2, 3, 4, 5];
        let weighted_index = WeightedIndex::new(weights).unwrap();
        let weighted_item_list = WeightedItemList {
            values,
            weighted_index,
        };
        let random_item: String = weighted_item_list.pick_random().to_string();
        assert!(weighted_item_list.values.contains(&random_item));
    }

    #[test]
    fn test_from_csv() {
        let csv = "value,weight\na,1\nb,2\nc,3\nd,4\ne,5\n";
        let weighted_item_list = WeightedItemList::from_csv(csv).unwrap();
        assert_eq!(weighted_item_list.values, vec!["a", "b", "c", "d", "e"]);
    }
}
