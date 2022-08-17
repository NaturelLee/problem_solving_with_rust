#![allow(dead_code)]

fn anagram_solution(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut alist = Vec::new();
    let mut blist = Vec::new();

    for c in s1.chars() {
        alist.push(c);
    }
    for c in s2.chars() {
        blist.push(c);
    }

    alist.sort();
    blist.sort();

    let mut matched = true;

    let mut pos: usize = 0;

    while pos < alist.len() && matched {
        if alist[pos] == blist[pos] {
            pos += 1;
        } else {
            matched = false;
        }
    }
    matched
}

fn disorder_strings(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut t1 = s1.chars().collect::<Vec<char>>();
    let mut t2 = s2.chars().collect::<Vec<char>>();
    t1.sort();
    t2.sort();

    t1 == t2
}

fn disorder_strings_char_count(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut a1 = [0; 26];
    let mut a2 = [0; 26];

    for c in s1.chars() {
        a1[c as usize - 97] += 1;
    }

    for c in s2.chars() {
        a2[c as usize - 97] += 1;
    }

    a1 == a2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_disorder_strings() {
        assert!(disorder_strings(
            "abcdefghijklmnopqrstuvwxyz",
            "abcdeklmnxyzopqrstufghijvw"
        ));
        assert!(disorder_strings("abc", "acb"));
        assert!(disorder_strings("", ""));
    }

    #[test]
    fn test_anagram_solution() {
        assert!(anagram_solution("abc", "acb"));
        assert!(anagram_solution("abc", "cba"));
        assert!(anagram_solution("", ""));
        assert!(anagram_solution(
            "abcdefghijklmnopqrstuvwxyz",
            "abcdeklmnxyzopqrstufghijvw"
        ));
    }

    #[test]
    fn test_disorder_strings_char_count() {
        assert!(disorder_strings_char_count("abc", "acb"));
        assert!(disorder_strings_char_count("abc", "cba"));
        assert!(disorder_strings_char_count("", ""));
        assert!(disorder_strings_char_count(
            "abcdefghijklmnopqrstuvwxyz",
            "abcdeklmnxyzopqrstufghijvw"
        ));
    }
}
