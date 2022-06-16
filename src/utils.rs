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
