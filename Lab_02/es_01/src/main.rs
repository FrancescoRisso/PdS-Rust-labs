mod my_slug;
use my_slug::*;

const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

fn conv(c: char) -> char {
    let subs_i_as_vec: Vec<char> = SUBS_I.chars().collect();
    match subs_i_as_vec.iter().position(|&r| r == c) {
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
        wip.remove(wip.len() - 1);
        return wip;
    }

    wip
}

fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true
    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice
}
