use itertools::Itertools;

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
}
