use std::ops::DivAssign;

pub fn extract_prefix(file_name: &str) -> String {
    file_name.split('_').nth(0).unwrap().to_owned()
}

//creates string with as many zeroes, as their are zeroes in prefix in migration file name
pub fn construct_prefix(num: i32, prefix_len: usize) -> String {
    let digits = count_digits(num);
    if digits > prefix_len {
        return String::new();
    }
    let mut prefix = String::new();
    for _ in 0..prefix_len - count_digits(num) {
        prefix.push('0')
    }
    prefix.push_str(&num.to_string());
    prefix
}

//
pub fn change_prefix(prefix: &str, old_name: &str) -> String {
    let mut old_name_vec: Vec<String> = old_name.split("_").map(|x| x.to_owned()).collect();
    if old_name_vec.len() > 1 {
        old_name_vec.remove(0);
    }
    old_name_vec.insert(0, prefix.to_owned());
    let a = String::new();
    old_name_vec
        .iter()
        .fold(a, |a, b| a + b.as_str() + "_")
        .trim_end_matches(|a| a == '_')
        .to_owned()
}

pub fn count_digits(mut x: i32) -> usize {
    let mut n = 0;
    while x > 0 {
        n += 1;
        x.div_assign(10);
    }
    n
}

mod tests {
    use crate::migration::prefix;
    #[test]
    fn test_count_digits() {
        assert_eq!(prefix::count_digits(1), 1);
        assert_eq!(prefix::count_digits(10), 2);
        assert_eq!(prefix::count_digits(100), 3);
        assert_eq!(prefix::count_digits(12345), 5);
    }

    #[test]
    fn test_extract_prefix() {
        assert_eq!(prefix::extract_prefix("001_migration.sql"), "001");
        assert_eq!(prefix::extract_prefix("1_migration.sql"), "1");
        assert_eq!(prefix::extract_prefix("002_migration.sql"), "002");
        assert_eq!(
            prefix::extract_prefix("001migration.sql"),
            "001migration.sql"
        );
        assert_eq!(prefix::extract_prefix(""), "");
        assert_eq!(prefix::extract_prefix("5000_migration.sql"), "5000");
    }

    #[test]
    fn test_construct_prefix() {
        assert_eq!(prefix::construct_prefix(1, 2), "01");
        assert_eq!(prefix::construct_prefix(2, 5), "00002");
        assert_eq!(prefix::construct_prefix(135, 2), "");
        assert_eq!(prefix::construct_prefix(0, 0), "0");
    }

    #[test]
    fn test_change_prefix() {
        assert_eq!(
            prefix::change_prefix("0002", "001_migration.sql"),
            "0002_migration.sql"
        );
        assert_eq!(
            prefix::change_prefix("02", "1_migration.sql"),
            "02_migration.sql"
        );
        assert_eq!(
            prefix::change_prefix("10", "0_migration.sql"),
            "10_migration.sql"
        );
        assert_eq!(
            prefix::change_prefix("0100", "migration.sql"),
            "0100_migration.sql"
        );
        assert_eq!(
            prefix::change_prefix("000002", "_migration.sql"),
            "000002_migration.sql"
        );
    }
}
