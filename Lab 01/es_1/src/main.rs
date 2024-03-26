const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

fn conv(c: char) -> char {
    match SUBS_I.find(c) {
        Some(index) => SUBS_O.chars().nth(index).unwrap(),
        None => '-',
    }
}

fn slugify(s: &str) -> String {
    let mut wip = String::new();

    for ch in s.to_lowercase().chars() {
        if "qwertyuiopasdfghjklzxcvbnm1234567890".contains(ch) {
            wip.push(ch);
        } else {
            let new = conv(ch);
            if new != '-' || !wip.ends_with('-') {
                wip.push(conv(ch));
            }
        }
    }

    if wip == "-".to_string() {
        return wip;
    }

    if wip.ends_with('-') {
		wip.remove(wip.len()-1);
		return wip;
	}

	wip
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_accent() {
        assert_eq!(slugify("à"), "a");
    }

    #[test]
    fn convert_non_accent() {
        assert_eq!(slugify("a"), "a");
    }

    #[test]
    fn convert_unknown() {
        assert_eq!(slugify("`"), "-");
    }

    #[test]
    fn convert_accent_unknown() {
        assert_eq!(slugify("ķ"), "-");
    }

    #[test]
    fn convert_with_space() {
        assert_eq!(slugify("a b"), "a-b");
    }

    #[test]
    fn convert_string_with_accents() {
        assert_eq!(slugify("Mammà"), "mamma");
    }

    #[test]
    fn convert_emtpy() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn convert_more_spaces() {
        assert_eq!(slugify("a        b"), "a-b");
    }

    #[test]
    fn convert_multiple_invalid() {
        assert_eq!(slugify("a???"), "a");
    }

    #[test]
    fn convert_all_invalid() {
        assert_eq!(slugify("???"), "-");
    }

    #[test]
    fn convert_ending_space() {
        assert_eq!(slugify("Bah "), "bah");
    }
}

fn main() {
    slugify("todo");
}
