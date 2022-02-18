pub fn first_word(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    str.len()
}

pub fn first_word_str_slice(str_slice: &str) -> usize {
    for (i, &item) in str_slice.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    str_slice.len()
}