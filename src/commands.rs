use clap::{arg, Command};

const ITEM_TYPES: [&str; 6] = ["account", "transaction", "payment", "saving", "queued", "msi"];

pub fn cli() -> Command<'static> {
    Command::new("wallet")
        .about("CLI app for bank accounts management.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        // Account subcommands.
        .subcommand(
            Command::new("account")
                .about("Account related subcommands.")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("active")
                        .about("Set one account as the active account. *This deactivates all other accounts.")
                        .arg_required_else_help(true)
                        .arg(arg!(-i --id <ID> "ID of the account to set active."))
                )
                .subcommand(
                    Command::new("edit")
                        .about("Edit the data of an account.")
                        .arg_required_else_help(true)
                        .args([
                            arg!(-i --id <ID> "ID of the account to edit."),
                            arg!(-n --name <NAME> "New name to the account.").default_value("keep"),
                            arg!(-b --balance <BALANCE> "New balance of the account.").default_value("keep")
                        ])
                )
                .subcommand(
                    Command::new("transfer")
                        .about("Transfer balance to another account.")
                        .arg_required_else_help(true)
                        .args([
                            arg!(-b --balance <BALANCE> "Balance to transfer."),
                            arg!(-d --destination <DESTINATION> "ID of the destination account."),
                            arg!(-s --source <SOURCE> "ID of the source account.").default_value("active")
                        ])
                )
        )
        // Backup subcommand.
        .subcommand(
            Command::new("backup")
                .about("Creates a copy of the current database.")
                .arg_required_else_help(true)
                .arg(arg!([FILENAME] "File name of the backup database."))
        )
        // Delete subcommand.
        .subcommand(
            Command::new("delete")
                .about("Delete items from database.")
                .arg_required_else_help(true)
                .args(&[
                    arg!([ITEM] "Item type to delete.").possible_values(ITEM_TYPES),
                    arg!(-i --id [ID] "ID of the item to delete.").default_value("0"),
                    arg!(--all "Deletes all items in table.")
                ])
        )
        // Init subcommand.
        .subcommand(
            Command::new("init")
                .about("Creates and initialize the database.")
        )
        // List subcommand.
        .subcommand(
            Command::new("list")
                .about("List specified items.")
                .arg_required_else_help(true)
                .args([
                    arg!([ITEM] "Item type to list.").possible_values(ITEM_TYPES),
                    arg!(-c --count [COUNT] "Number of items required to list.").default_value("10"),
                    arg!(--all "List all items in table")
                ])
        )
        // New subcommands.
        .subcommand(
            Command::new("new")
                .about("Add new stuff to database (account, expense, incomming, etc.).")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("account")
                        .about("Creates a new account in the database.")
                        .arg_required_else_help(true)
                        .args(&[
                            arg!([NAME] "Account name."),
                            arg!([BALANCE] "Initial balance.").default_value("0")
                        ])
                )
                .subcommand(
                    Command::new("expense")
                        .about("Add new expense to database.")
                        .arg_required_else_help(true)
                        .args(&[
                            arg!([MESSAGE] "Message of the expense."),
                            arg!([VALUE] "Value of the expense."),
                            arg!(-c --charged "Add this if the expense is already charged in the account.")
                        ])
                )
                .subcommand(
                    Command::new("incoming")
                        .about("Add new incomming to database.")
                        .arg_required_else_help(true)
                        .args(&[
                            arg!([MESSAGE] "Message of the incomming."),
                            arg!([VALUE] "Value of the incomming.")
                        ])
                )
                .subcommand(
                    Command::new("queue")
                        .about("Add future expenses to queue. Not recommended.")
                        .arg_required_else_help(true)
                        .args(&[
                            arg!([MESSAGE] "Message of the expense to queue."),
                            arg!([VALUE] "Value of the expense to queue."),
                            arg!(--dequeue "Execute all queued items as expenses.")
                        ])
                )
        )
}
