use anyhow::{Error, Result};
use csv::ReaderBuilder;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::Deserialize;

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

    /// Convert the values in a CSV to a Vec of WeightedItem structs.
    /// The CSV must have a header row with the columns "value" and "weight".
    /// Note that these headers are case-sensitive, but the order of the columns does not matter.
    pub fn from_csv(csv: &str) -> Result<WeightedItemList> {
        // Check that the CSV is not empty.
        if csv.is_empty() {
            return Err(Error::msg("The CSV string is empty."));
        }

        // Build the csv reader
        let mut csv_reader = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_reader(csv.as_bytes());

        // Get the headers
        let headers = csv_reader.headers()?;

        // No empty headers
        if headers.iter().any(|header| header.is_empty()) {
            return Err(Error::msg("The CSV string has an empty header."));
        }

        // One header must be "value" and the other must be "weight"
        if let Some(header) = headers
            .iter()
            .find(|header| *header != "value" && *header != "weight")
        {
            return Err(Error::msg(format!(
                "The CSV string has an invalid header: {}.",
                header
            )));
        }

        // Build a vector of WeightedItem structs from the CSV.
        // Skip any rows that have an empty value or a weight of 0.
        let mut csv_rows = Vec::new();
        for result in csv_reader.deserialize() {
            let record: WeightedItem = result?;
            if record.value.is_empty() || record.weight == 0 {
                continue;
            }
            csv_rows.push(record);
        }

        let values: Vec<String> = csv_rows.iter().map(|item| item.value.clone()).collect();
        let weights: Vec<u32> = csv_rows.iter().map(|item| item.weight).collect();

        // Create a WeightedIndex from the weights vector.
        let weighted_index: WeightedIndex<u32> =
            WeightedIndex::new(weights).map_err(|_| Error::msg("Invalid weights."))?;

        Ok(WeightedItemList {
            values,
            weighted_index,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]
    use super::*;

    #[test]
    fn test_pick_random() {
        // Should pick a random item from the list.
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
        // Should create a WeightedItemList from a CSV string.
        let csv = "value,weight\na,1\nb,2\nc,3\nd,4\ne,5\n";
        let weighted_item_list = WeightedItemList::from_csv(csv).unwrap();
        assert_eq!(weighted_item_list.values, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_column_order() {
        // Should be able to handle different orders of the headers.
        let csv = "weight,value\n1,a\n2,b\n3,c\n4,d\n5,e\n";
        let weighted_item_list = WeightedItemList::from_csv(csv).unwrap();
        assert_eq!(weighted_item_list.values, vec!["a", "b", "c", "d", "e"]);

        let csv = "value,weight\na,1\nb,2\nc,3\nd,4\ne,5\n";
        let weighted_item_list = WeightedItemList::from_csv(csv).unwrap();
        assert_eq!(weighted_item_list.values, vec!["a", "b", "c", "d", "e"]);
    }
}
