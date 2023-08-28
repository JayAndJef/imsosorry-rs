use itertools::Itertools;
use regex::Regex;

pub fn uwuify(text: &str) -> String {
    let text = words_replace(text);
    let text = replace_with_w(&text);
    nyaify(&text)
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

fn nyaify(text: &str) -> String {
    let nyai_regex = Regex::new(r"(?<start>n)(?<prec>[aeou][^aeiou])").unwrap();
    nyai_regex.replace_all(text, "${start}y$prec").to_string()
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

    #[test]
    fn test_nyai() {
        let n_replaced = nyaify("none is good");
        assert_eq!(n_replaced, "nyone is good");
    }

    #[test]
    fn test_uwu() {
        let uwued_text = uwuify("According to all known laws of aviation, there is no way a bee should be able to fly.");
        assert_eq!(uwued_text, "Accowding to aww knyown waws of aviation, thewe is nyo way a bee shouwd be abwe to fwy.")
    }
}
