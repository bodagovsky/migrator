pub(crate) mod create;
pub mod prefix;
pub mod tools;
use std::fs;
use std::io::Write;
use std::path::Path;

const GOOSE: &str = "-- +goose Up
-- SQL in this section is executed when the migration is applied.

-- +goose Down
-- SQL in this section is executed when the migration is rolled back.";

//return last number of migration numeration and its length with prefixed zeroes
pub(crate) fn last_number(migrations: Vec<String>) -> (i32, usize) {
    let first = migrations[migrations.len() - 1].split('_').nth(0).unwrap();
    let n: i32 = first.parse().unwrap();
    (n, first.len())
}

pub(crate) fn create_sql_migration(num: i32, prefix_len: usize, file_name: &str) {
    let prefix = prefix::construct_prefix(num, prefix_len);
    let title = String::from("migrations/") + prefix.as_str() + "_" + file_name + ".sql";
    let file = fs::File::create(Path::new(title.as_str()));
    match file {
        Ok(mut f) => {
            f.write(GOOSE.as_bytes());
        }
        Err(e) => {
            print!("{}", e.to_string());
        }
    };
}

pub(crate) fn sort_migrations() {
    match tools::get_migrations() {
        Ok(files) => {
            let current_dir: String = tools::extract_migrations_path();
            let first_file_name: String = tools::file_name(&files[0]);
            let old_prefix: String = prefix::extract_prefix(&first_file_name);
            let mut serial_number: i32 = 0;
            let prefix_len = old_prefix.len();
            if let Ok(num) = old_prefix.parse() {
                serial_number = num;
            } else {
                println!("Failed to get serial number from {}", serial_number);
                return;
            }
            for file in &files[1..] {
                serial_number += 1;
                let file_name = tools::file_name(file);
                let prefix = prefix::extract_prefix(&file_name);
                let num: Result<i32, std::num::ParseIntError> = prefix.parse();
                match num {
                    Ok(n) => {
                        if n == serial_number {
                            continue;
                        } else {
                            let new_prefix = prefix::construct_prefix(serial_number, prefix_len);
                            let new_file_name = prefix::change_prefix(&new_prefix, &file_name);
                            fs::rename(
                                current_dir.clone() + &file_name,
                                current_dir.clone() + &new_file_name,
                            );
                        }
                    }
                    Err(e) => println!("Failed to parse serial number: {}", e.to_string()),
                }
            }
        }
        Err(e) => println!("{}", e.to_string()),
    }
}

mod tests {
    use crate::migration::sort_migrations;

    #[test]
    fn test_renaming() {
        sort_migrations()
    }
}
