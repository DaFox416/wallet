# Wallet CLI
Wallet cli app for bank accounts management.

The following README describes how to use the wallet CLI app.

## Installation

TODO

## First steps

***NOTE:*** *If you are running the app from source, you're probably using  **cargo**, so replace all the ´wallet´ instances with ´cargo run´.*

The first step to start using wallet is to initialize the database, for this we will run this command:

    wallet init

After this, you should see the next message:

    Successfully created new database!

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

    1   .- * Banorte              $        5000.00 ->         5000.00
    2   .-   BBVA Bancomer        $        2000.57 ->         2000.57
    3   .-   Banamex              $           0.00 ->            0.00

When you list an account it will appears in this line format. Take a look to the first account:

    1   .- * Banorte              $        5000.00 ->         5000.00

The first number is its ID in the database. Notice that only this account has a star symbol (__*__) before its name indicating that this is the default account, this is the account all transactions are added to by default. Then you'll see the account name whose length must be less than 20 characters.

Finally you'll see two values: the first one is the balance in the account and must match with the real balance in your bank account; and the second value is the available balance of the account. The available balance must always be equal to or less than the account balance. The available balance could be less than account balance when you register a transaction that is not charged to the account yet, for example: when you have planned or made a purchase, but the money stills in your account, so you must not spend that money in other purchase. in these cases the balance of your account has money that already corresponds to a purchase, so that money is no longer available, resulting in an available balance less than the account balance.

Now, you can change your default account with the **account default** subcommand:

    wallet account default <ID>

Let's set the account 'BBVA Bancomer' as the default account. Run this command:

    wallet account default 2

Now if we run again the **list** subcommand we'll get this output:

    1   .-   Banorte              $        5000.00 ->         5000.00
    2   .- * BBVA Bancomer        $        2000.57 ->         2000.57
    3   .-   Banamex              $           0.00 ->            0.00
