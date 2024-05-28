use regex::Regex;
use crate::data_structures::Wholegame;

/// Applies specified transformations to a `Wholegame` data structure based on the provided rule.
///
/// # Arguments
/// * `wholegame` - A mutable reference to the `Wholegame` instance to apply transformations on.
/// * `rule` - A string slice that specifies the transformation rule to apply.
///
/// # Supported Rules
/// - `"fullname"`: Concatenates first names and surname into a full name.
/// - `"team_name_regex"`: Applies a regex pattern to transform the team name.
///
/// # Examples
/// ```
/// use your_crate::data_structures::Wholegame;
/// use your_crate::apply_transformations;
///
/// let mut game = Wholegame::new();
/// game.set_first_names("Alice".to_string());
/// game.set_surname("Liddell".to_string());
/// game.set_team("U10 Wonderland".to_string());
///
/// apply_transformations(&mut game, "fullname");
/// assert_eq!(game.get_full_name(), "Alice Liddell");
///
/// apply_transformations(&mut game, "team_name_regex");
/// assert_eq!(game.get_team(), "U10 Wonderland");
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::Wholegame;

    #[test]
    fn test_apply_fullname_transformation() {
        let mut game = Wholegame::new();
        game.set_first_names("Alice".to_string());
        game.set_surname("Liddell".to_string());
        
        apply_transformations(&mut game, "fullname");
        assert_eq!(game.get_full_name(), "Alice Liddell");
    }

    #[test]
    fn test_apply_team_name_regex_transformation() {
        let mut game = Wholegame::new();
        game.set_team("U10 Wonderland".to_string());
        
        apply_transformations(&mut game, "team_name_regex");
        assert_eq!(game.get_team(), "U10 Wonderland");
    }
}
