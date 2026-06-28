use std::collections::HashMap;

#[derive(Debug)]
pub struct LSystem {
    replacements: HashMap<String, String>,
    start: String,
}

pub fn new(replacements: &[(&str, &str)], start: &str) -> LSystem {
    return LSystem {
        replacements: replacements
            .iter()
            .cloned()
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect(),
        start: String::from(start),
    };
}

impl LSystem {
    pub fn expand(&self, n: i32) -> String {
        let mut curr_string = self.start.clone();
        for _ in 0..n {
            curr_string = self.expand_step(&curr_string);
        }

        curr_string
    }

    fn expand_step(&self, curr: &str) -> String {
        let mut new_str = String::new();

        let mut i = 0;

        for c in curr.chars() {
            let found = self
                .replacements
                .iter()
                .find_map(|(k, v)| curr[i..].starts_with(k).then_some(v));

            if let Some(val) = found {
                new_str += val;
            } else {
                new_str.push(c);
            }

            i += c.len_utf8();
        }

        String::from(new_str)
    }
}

#[test]
fn test_expansion_step() {
    let system = LSystem {
        replacements: HashMap::from([
            (String::from("A"), String::from("AB")),
            (String::from("B"), String::from("A")),
        ]),
        start: String::from("AB"),
    };

    assert_eq!(String::from("ABAABABA"), system.expand_step("ABAAB"))
}

#[test]
fn test_expansion() {
    let system = LSystem {
        replacements: HashMap::from([
            (String::from("A"), String::from("AB")),
            (String::from("B"), String::from("A")),
        ]),
        start: String::from("A"),
    };

    assert_eq!(String::from("ABAABABA"), system.expand(4))
}
