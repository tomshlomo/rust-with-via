use std::collections::HashMap;

pub struct Inventory {
    items: HashMap<String, u32>,
}

// todo: implement the `From<Vec<(String, u32)>>` trait for `Inventory`.

// todo: implement the `Add<Inventory>` trait for `Inventory`.

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
