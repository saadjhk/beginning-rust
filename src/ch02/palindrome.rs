pub fn check_palindrome(word: &str) -> bool {
    let word_length = word.len() / 2;
    let mut st_ind = 0; let mut end_ind = word.len() - 1;

    while st_ind < word_length {
        if word.chars().nth(st_ind).unwrap() != word.chars().nth(end_ind).unwrap() {return false;}

        end_ind = end_ind - 1;
        st_ind = st_ind + 1;
    }

    return true;
}