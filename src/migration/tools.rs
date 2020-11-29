use crate::migration::prefix;
use std::path::Path;
use std::{env, fs, io};

pub(crate) fn ls() -> Result<Vec<String>, io::Error> {
    let mut entries = fs::read_dir("./migrations")?
        .map(|res| res.map(|e| e.file_name().into_string().unwrap()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort_by(|a, b| {
        prefix::extract_prefix(&file_name(&a))
            .parse::<i32>()
            .unwrap()
            .cmp(&prefix::extract_prefix(&file_name(&b)).parse().unwrap())
    });
    if entries.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "empty folder"));
    }
    Ok(entries)
}

pub(crate) fn get_migrations() -> Result<Vec<String>, io::Error> {
    let mut entries = fs::read_dir("./migrations")?
        .map(|res| res.map(|e| e.path().to_str().unwrap().to_owned()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort_by(|a, b| {
        prefix::extract_prefix(&file_name(&a))
            .parse::<i32>()
            .unwrap()
            .cmp(&prefix::extract_prefix(&file_name(&b)).parse().unwrap())
    });
    if entries.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "migrations folder is empty",
        ));
    }
    Ok(entries)
}

pub(crate) fn file_name(path: &str) -> String {
    match Path::new(path).file_name() {
        Some(file) => match file.to_str() {
            Some(name) => String::from(name),
            None => String::new(),
        },
        None => String::new(),
    }
}

pub(crate) fn extract_migrations_path() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_owned() + "/migrations/"
}

mod tests {
    use crate::migration::tools;

    #[test]
    fn test_file_name() {
        assert_eq!(
            tools::file_name("Users/Documents/migrations/001_migrations.sql"),
            "001_migrations.sql"
        );
        assert_eq!(
            tools::file_name("/002_migrations.sql"),
            "002_migrations.sql"
        );
        assert_eq!(tools::file_name("003_migrations.sql"), "003_migrations.sql");
        assert_eq!(
            tools::file_name("004_migrations.sql/"),
            "004_migrations.sql"
        );
    }

    #[test]
    #[ignore]
    fn test_extract_migration_path() {
        assert_eq!(
            tools::extract_migrations_path(),
            "/Users/abfdogovsky/Documents/migrator/migrations/"
        )
    }
}
