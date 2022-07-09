# Wallet CLI
Wallet cli app for bank accounts management.

The following README describes how to use the wallet CLI app.

## Installation

TODO

## First steps

The first step to start using wallet is to initialize the database, for this we will run this command:

    wallet init

After this, you should see the next message:

    # Successfully created new database!

And now you are able to start with the accounts creation.

## Accounts management

You can create your first account with the **new account** subcommand:

    wallet new account <NAME> [BALANCE=0.0]

This commands receives two arguments, the name of the account and optional its initial balance (Initial balance is 0.0 by default). For example:

    wallet new account Banorte 5000
    wallet new account "BBVA Bancomer" 2000.57
    wallet new account Banamex

Feel free to create your own accounts.

You can see your current accounts by using the **list** subcommand:

    wallet list <ITEM_TYPE> [COUNT=10] [--all]

The list subcommand can show any item from your database, but for now we'll use it for see our accounts:

    wallet list account

In this step you should see the following output:

    1   .- Banorte              $        5000.00     Active
    2   .- BBVA Bancomer        $        2000.57   Inactive
    3   .- Banamex              $           0.00   Inactive

Notice that your first registered account appears as *Active*. This is the account all transactions are added to by default.

You can change your active account with the **account active** subcommand:

    wallet account active <ID>

Let's set the account 'BBVA Bancomer' as the active account. Run this command:

    wallet account active 2

Now if we run again the **list** subcommand we'll get this output:

    1   .- Banorte              $        5000.00   Inactive
    2   .- BBVA Bancomer        $        2000.57     Active
    3   .- Banamex              $           0.00   Inactive
