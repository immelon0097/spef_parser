mod spef_parser;

fn main() {
    let spef_file_str = "/home/immelon/projects/iPD/src/database/manager/parser/spef/spef-parser/aes_simple.spef";
    let parse_result = spef_parser::parse_spef_file(spef_file_str);
    match parse_result {
        Ok(exchange_data) => {
            println!("Parsed {spef_file_str} successfully\n, exchange_data: {exchange_data:#?}");
        }
        Err(_) => todo!(),
    }
}
