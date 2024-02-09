use pgrx::pg_extern;

#[pg_extern]
pub fn pad_left(s: &str, total_len: i32, pad: Option<&str>) -> String {
    let pad_str = pad.filter(|p| !p.is_empty()).unwrap_or(" ");
    let s_len = s.chars().count() as i32;
    let padding_needed = if total_len > s_len {
        total_len - s_len
    } else {
        0
    } as usize;

    let mut padded_string = String::new();
    let pad_chars: Vec<char> = pad_str.chars().collect();
    let pad_len = pad_chars.len();
    for i in 0..padding_needed {
        padded_string.push(pad_chars[i % pad_len]);
    }

    padded_string + s
}

#[pg_extern]
pub fn pad_right(s: &str, total_len: i32, pad: Option<&str>) -> String {
    let pad_str = pad.filter(|p| !p.is_empty()).unwrap_or(" ");
    let s_len = s.chars().count() as i32;
    let padding_needed = if total_len > s_len {
        total_len - s_len
    } else {
        0
    } as usize;

    s.to_owned()
        + &pad_str.repeat(padding_needed / pad_str.len())
        + &pad_str[0..padding_needed % pad_str.len()]
}
#[pg_extern]
fn pad_both(value: &str, length: i32, pad: Option<&str>) -> String {
    let pad_len = length as usize;
    let value_len = value.chars().count();

    if value_len >= pad_len {
        return value.to_string();
    }

    let pad_str = pad.unwrap_or(" ");
    let total_padding = pad_len - value_len;
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;

    let left_pad = pad_str.repeat(left_padding / pad_str.chars().count())
        + &pad_str
            .chars()
            .take(left_padding % pad_str.chars().count())
            .collect::<String>();
    let right_pad = pad_str.repeat(right_padding / pad_str.chars().count())
        + &pad_str
            .chars()
            .take(right_padding % pad_str.chars().count())
            .collect::<String>();

    format!("{}{}{}", left_pad, value, right_pad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_right_with_custom_pad() {
        let result = pad_right("James", 10, Some("-"));
        assert_eq!(result, "James-----");
    }

    #[test]
    fn test_pad_right_with_space() {
        let result = pad_right("James", 10, None);
        assert_eq!(result, "James     ");
    }

    #[test]
    fn test_pad_right_no_padding_needed() {
        let result = pad_right("James", 5, Some("-"));
        assert_eq!(result, "James");
    }

    #[test]
    fn test_pad_right_empty_string() {
        let result = pad_right("", 5, Some("-"));
        assert_eq!(result, "-----");
    }

    #[test]
    fn test_pad_right_short_pad_string() {
        let result = pad_right("James", 9, Some("-="));
        assert_eq!(result, "James-=-=");
    }
    #[test]
    fn test_pad_both_with_custom_pad() {
        let result = pad_both("James", 10, Some("_"));
        assert_eq!(result, "__James___");
    }

    #[test]
    fn test_pad_both_with_space() {
        let result = pad_both("James", 10, None);
        assert_eq!(result, "  James   ");
    }

    #[test]
    fn test_pad_both_no_padding_needed() {
        let result = pad_both("James", 5, Some("_"));
        assert_eq!(result, "James");
    }

    #[test]
    fn test_pad_both_empty_string() {
        let result = pad_both("", 5, Some("_"));
        assert_eq!(result, "_____");
    }

    #[test]
    fn test_pad_both_short_pad_string() {
        let result = pad_both("James", 11, Some("_-"));
        assert_eq!(result, "_-_James_-_");
    }
    #[test]
    fn test_basic_padding() {
        assert_eq!(pad_left("hello", 10, None), "     hello");
    }

    #[test]
    fn test_no_padding_needed() {
        assert_eq!(pad_left("hello", 5, None), "hello");
        assert_eq!(pad_left("hello", 4, None), "hello");
    }

    #[test]
    fn test_custom_padding_character() {
        assert_eq!(pad_left("hello", 10, Some("-")), "-----hello");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(pad_left("", 5, None), "     ");
    }

    #[test]
    fn test_padding_with_empty_pad_string() {
        assert_eq!(pad_left("hello", 10, Some("")), "     hello");
    }

    #[test]
    fn test_long_pad_string() {
        assert_eq!(pad_left("hello", 10, Some("ab")), "ababahello");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(pad_left("你好", 4, Some("界")), "界界你好");
    }
}
