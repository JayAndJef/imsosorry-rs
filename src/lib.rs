use itertools::Itertools;
use regex::Regex;

pub fn uwuify(text: &str, stutter_strength: Option<f32>, emoji_strength: Option<f32>, tilde_strength: Option<f32>) {
    todo!()
}

fn words_replace(text: &str) -> String {
    text
        .split(" ")
        .map(replace_word)
        .join(" ")
        
}

fn replace_word(word: &str) -> &str {
    match word {
        "small" => "smol",
        "cute" => "kawaii~",
        "fluff" => "floof",
        "love" => "luv",
        "stupid" => "baka",
        "idiot" => "baka",
        "what" => "nani",
        "meow" => "nya~",
        "roar" => "rawrr~",
        _ => word,
    }
}

fn replace_with_w(text: &str) -> String {
    let char_regex = Regex::new("[lr]").unwrap();

    char_regex.replace_all(text, "w").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_works() {
        assert_eq!(1, 1)
    }

    #[test]
    fn text_word_replace() {
        let replaces_string = words_replace("small cute fluff love stupid idiot what meow roar");
        assert_eq!(replaces_string, "smol kawaii~ floof luv baka baka nani nya~ rawrr~")
    }

    #[test]
    fn test_replace_with_w() {
        let w_replace = replace_with_w("rowdy rowers are great");
        assert_eq!(w_replace, "wowdy wowews awe gweat");
    }
}
