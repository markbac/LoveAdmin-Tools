use regex::Regex;
use crate::data_structures::Wholegame;

pub fn apply_transformations(wholegame: &mut Wholegame, rule: &str) {
    if rule == "fullname" {
        let full_name = format!("{} {}", wholegame.get_first_names(), wholegame.get_surname());
        wholegame.set_full_name(full_name);
    } else if rule.starts_with("team_name_regex") {
        let re = Regex::new(r"U\d+ \w+").unwrap();  // Example pattern
        if let Some(caps) = re.captures(wholegame.get_team()) {
            wholegame.set_team(caps.get(0).map_or("", |m| m.as_str()).to_string());
        }
    }
}
