pub fn item_type_to_table_name(item_type: &str) -> String {
    match item_type {
        "account" | "transaction" | "payment" | "saving" => {
            format!("{}s", item_type).to_string()
        }
        "queued" | "msi" => {
            format!("{}_purchases", item_type).to_string()
        }
        _ => unreachable!()
    }
}

pub fn validate_tables(e_msg: &str, table_name: &str) {
    if e_msg.contains("no such table:") {
        println!("Table '{}' not found! Try 'wallet init' before use it.", table_name);
    } else {
        println!("Something went wrong with the query!");
    }
}

pub fn opt_str_to_opt_i64(opt_str: Option<&str>) -> Option<i64> {
    if let Some(str_value) = opt_str {
        let i64_value = match str_value.parse::<i64>() {
            Ok(value) => value,
            Err(_) => panic!("The value '{}' must be an integer value!", str_value)
        };

        Some(i64_value)
    } else {
        None
    }
}
