#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fmt::Write, str::FromStr};

    #[test]
    fn entry_test() {
        let mut map = HashMap::new();
        map.entry("fuck")
            .or_insert(String::from_str("you").unwrap());
        assert_eq!(&"you", map.get("fuck").unwrap());
        map.entry("fuck").and_modify(|you| {
            you.write_str(&"mu");
        });
        assert_eq!(&"youmu", map.get("fuck").unwrap());
    }
}
