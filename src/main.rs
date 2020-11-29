#![feature(crate_visibility_modifier)]

mod migration;
use clap::{App, Arg};
use migration::create::create_file_name;
use migration::tools::ls;
use migration::{create_sql_migration, last_number, sort_migrations};

const DEFAULT_LEN: usize = 5;

fn main() {
    let matches = App::new("Migrator")
        .author("Artur Bodagovsky")
        .about("Software Engineer")
        .arg(
            Arg::new("sort")
                .short('s')
                .long("sort")
                .help_heading(Option::from("sorts migrations in \"/migrations\" folder")),
        )
        .arg(
            Arg::new("file_name")
                .short('n')
                .long("name")
                .takes_value(true)
                .help_heading(Option::from("name of the new migration file")),
        )
        .get_matches();

    if matches.is_present("sort") {
        sort_migrations()
    }

    if let Some(name) = matches.value_of("file_name") {
        let file_name = create_file_name(name.split(" ").collect::<Vec<&str>>());
        if let Ok(files) = ls() {
            let (last, len) = last_number(files);
            create_sql_migration(last + 1, len, file_name.as_str())
        } else {
            create_sql_migration(1, DEFAULT_LEN, file_name.as_str())
        }
    }
}
