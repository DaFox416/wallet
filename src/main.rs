mod structs;
mod utils;
mod commands;
mod walletdb;

use std::path::PathBuf;

fn main() {
    let matches = commands::cli().get_matches();

    match matches.subcommand() {
        Some(("account", sub_matches)) => {
            let account_subcommands = sub_matches.subcommand().unwrap();

            match account_subcommands {
                ("active", args) => {
                    let id: i64 = args.value_of_t("ID").expect("Required...");

                    walletdb::account_active(id).unwrap();
                }
                ("edit", args) => {
                    println!("...");
                }
                ("transfer", args) => {
                    println!("...");
                }
                _ => unreachable!()
            }
        }
        Some(("backup", args)) => {
            let filename = args.value_of("FILENAME").expect("Required...");
            let backup_path = PathBuf::from(format!("./{}.db3", filename));

            walletdb::backup_database(&backup_path).unwrap();
        }
        Some(("delete", args)) => {
            let item_type = args.value_of("ITEM").expect("Required...");
            let id_item: i64 = args.value_of_t("id").expect("Required...");
            let delete_all = args.is_present("all");

            let table_name = utils::item_type_to_table_name(item_type);
            let id_name = format!("id_{}", item_type).to_string();

            walletdb::delete_items(
                &table_name, &id_name, id_item, delete_all
            ).unwrap();
        }
        Some(("init", _)) => {
            match walletdb::initialize_database() {
                Ok(_) => println!("Successfully created new database!"),
                Err(_) => println!("Something went wrong with the database creation! Try again...")
            }
        }
        Some(("list", args)) => {
            let item_type = args.value_of("ITEM").expect("Required...");
            let count: i64 = args.value_of_t("count").expect("Required...");

            let table_name = utils::item_type_to_table_name(item_type);

            match walletdb::list(&table_name, count) {
                Err(e) => utils::validate_tables(&format!("{}", e), &table_name),
                _ => ()
            }
        }
        Some(("new", sub_matches)) => {
            let new_subcommands = sub_matches.subcommand().unwrap();

            match new_subcommands {
                ("account", args) => {
                    let name = args.value_of("NAME").expect("Required...");
                    let balance: f64 = args.value_of_t("BALANCE").expect("Required...");

                    walletdb::new_account(&name, &balance).unwrap();
                }
                ("expense", args) => {
                    let message = args.value_of("MESSAGE").expect("Required...");
                    let value: f64 = args.value_of_t("VALUE").expect("Required...");
                    let charged = args.is_present("charged");

                    println!("New expense: {} - {:.2} - {}", &message, &value, &charged);
                }
                ("incoming", args) => {
                    let message = args.value_of("MESSAGE").expect("Required...");
                    let value: f64 = args.value_of_t("VALUE").expect("Required...");

                    println!("New incoming: {} - {:.2}", &message, &value);
                }
                _ => unreachable!()
            }
        }
        _ => println!("Not match yet!"),
    }
}
