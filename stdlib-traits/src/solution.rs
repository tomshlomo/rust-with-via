use std::collections::HashMap;
use std::convert::From;
use std::ops::Add;

pub struct Inventory {
    items: HashMap<String, u32>,
}

impl From<Vec<(String, u32)>> for Inventory {
    fn from(items: Vec<(String, u32)>) -> Self {
        let mut inventory = HashMap::new();
        for (name, qty) in items {
            inventory.insert(name, qty);
        }
        Self { items: inventory }
        // We will learn more about iterators in day 4, but here is a more idiomatic way to do the above:
        // Self {
        //     items: items.into_iter().collect(),
        // }
    }
}

impl Add for Inventory {
    type Output = Inventory;

    fn add(self, other: Inventory) -> Inventory {
        let mut combined = self.items;
        for (name, qty) in other.items {
            // This is a more idiomatic way to do the following:
            // match combined.get(&name) {
            //     Some(existing_qty) => {
            //         combined.insert(name, existing_qty + qty);
            //     }
            //     None => {
            //         combined.insert(name, qty);
            //     }
            // }
            *combined.entry(name).or_insert(0) += qty;
        }
        Self { items: combined }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec() {
        let items = vec![("apple".to_string(), 10), ("banana".to_string(), 20)];
        let inventory: Inventory = items.into();
        let expected: HashMap<String, u32> =
            [("apple".to_string(), 10), ("banana".to_string(), 20)]
                .iter()
                .cloned()
                .collect();
        assert_eq!(inventory.items, expected);
    }

    #[test]
    fn test_from_empty_vec() {
        let items: Vec<(String, u32)> = Vec::new();
        let inventory: Inventory = items.into();
        let expected: HashMap<String, u32> = HashMap::new();
        assert_eq!(inventory.items, expected);
    }

    #[test]
    fn test_add() {
        let items1 = vec![("apple".to_string(), 10), ("banana".to_string(), 20)];
        let items2 = vec![("banana".to_string(), 5), ("orange".to_string(), 15)];
        let inventory1: Inventory = items1.into();
        let inventory2: Inventory = items2.into();

        let combined_inventory = inventory1 + inventory2;

        let expected: HashMap<String, u32> = [
            ("apple".to_string(), 10),
            ("banana".to_string(), 25),
            ("orange".to_string(), 15),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(combined_inventory.items, expected);
    }

    #[test]
    fn test_add_empty_inventories() {
        let inventory1: Inventory = Vec::new().into();
        let inventory2: Inventory = Vec::new().into();

        let combined_inventory = inventory1 + inventory2;

        let expected: HashMap<String, u32> = HashMap::new();
        assert_eq!(combined_inventory.items, expected);
    }

    #[test]
    fn test_add_non_empty_with_empty_inventory() {
        let items = vec![("apple".to_string(), 10), ("banana".to_string(), 20)];
        let inventory1: Inventory = items.into();
        let inventory2: Inventory = Vec::new().into();

        let combined_inventory = inventory1 + inventory2;

        let expected: HashMap<String, u32> =
            [("apple".to_string(), 10), ("banana".to_string(), 20)]
                .iter()
                .cloned()
                .collect();
        assert_eq!(combined_inventory.items, expected);
    }

    #[test]
    fn test_add_empty_with_non_empty_inventory() {
        let items = vec![("apple".to_string(), 10), ("banana".to_string(), 20)];
        let inventory1: Inventory = Vec::new().into();
        let inventory2: Inventory = items.into();

        let combined_inventory = inventory1 + inventory2;

        let expected: HashMap<String, u32> =
            [("apple".to_string(), 10), ("banana".to_string(), 20)]
                .iter()
                .cloned()
                .collect();
        assert_eq!(combined_inventory.items, expected);
    }
}
