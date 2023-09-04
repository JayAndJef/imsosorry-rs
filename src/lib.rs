use itertools::Itertools;
use regex::Regex;

#[must_use]
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
        "stupid" | "idiot" => "baka",
        "what" => "nani",
        "meow" => "nya~",
        "roar" => "rawrr~",
        _ => word,
    }
}

fn replace_with_w(text: &str) -> String {
    let char_regex = Regex::new("[lr]").unwrap();
    let ou_regex = Regex::new("[ou]").unwrap();

    let temp = char_regex.replace_all(text, "w").to_string();
    ou_regex.replace_all(&temp, "${0}w$0").to_string()
}

fn nyaify(text: &str) -> String {
    let nyai_regex = Regex::new(r"n([aeou][^aeiou])").unwrap();
    nyai_regex.replace_all(text, "ny$1").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_works() {
        assert_eq!(1, 1);
    }

    #[test]
    fn text_word_replace() {
        let replaces_string = words_replace("small cute fluff love stupid idiot what meow roar");
        assert_eq!(replaces_string, "smol kawaii~ floof luv baka baka nani nya~ rawrr~");
    }

    #[test]
    fn test_replace_with_w() {
        let w_replace = replace_with_w("rowdy rowers are great");
        assert_eq!(w_replace, "wowowdy wowowews awe gweat");
    }

    #[test]
    fn test_nyai() {
        let n_replaced = nyaify("none is good");
        assert_eq!(n_replaced, "nyone is good");
    }

    #[test]
    fn test_uwu() {
        let uwued_text = uwuify("According to all known laws of aviation, there is no way a bee should be able to fly.");
        assert_eq!(uwued_text, "Accowowding towo aww knyowown waws owof aviatiowon, thewe is nyowo way a bee showouwuwd be abwe towo fwy.");
    }
}
