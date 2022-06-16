use clap::{arg, Command};

pub fn cli() -> Command<'static> {
    Command::new("wallet")
        .about("CLI app for bank accounts management.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("backup")
                .about("Creates a copy of the current database.")
                .arg_required_else_help(true)
                .arg(arg!([FILENAME] "File name of the backup database."))
        )
        .subcommand(
            Command::new("delete")
                .about("Delete items from database.")
                .arg_required_else_help(true)
                .args(&[
                    arg!([ITEM] "Item to delete.")
                        .possible_values(["account", "transaction", "service", "queued", "msi"]),
                    arg!(-i --id [ID] "ID of the item to delete.").default_value("0"),
                    arg!(--all "Deletes all items in table.")
                ])
        )
        .subcommand(
            Command::new("init")
                .about("Creates and initialize the database.")
        )
        .subcommand(
            Command::new("list")
                .about("List specified items.")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .allow_external_subcommands(true)
                .subcommand(
                    Command::new("accounts")
                        .about("List the accounts in database.")
                )
                .subcommand(
                    Command::new("expenses")
                        .about("List the expenses of the current month.")
                )
        )
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
}
