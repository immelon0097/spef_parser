pub mod spef_data;

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "spef_parser/grammar/spef.pest"]

struct SpefParser;

/// process float data.
fn process_float(pair: Pair<Rule>) -> Result<f64, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();

    // remove the preceding "*" before the index
    let mut pair_str = pair.as_str();
    let clearned_str: String = pair_str.chars().filter(|&c| c != '*').collect();

    match clearned_str.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Failed to parse float".into() },
            pair_clone.as_span(),
        )),
    }
}

/// process xy coordinates
fn process_coordinates(pair: Pair<Rule>) -> Result<(f64, f64), pest::error::Error<Rule>> {
    let pair_clone = pair.clone();

    let tuple_pair = pair.clone();

    let mut inner_rules = pair_clone.into_inner();
    let x_coordiante_pair = inner_rules.next();
    let y_coordinate_pair = inner_rules.next();

    match (x_coordiante_pair, y_coordinate_pair) {
        (Some(x_float_pair), Some(y_float_pair)) => {
            let x_float = process_float(x_float_pair);
            let y_float = process_float(y_float_pair);
            match (x_float, y_float) {
                (Ok(x), Ok(y)) => Ok((x, y)),
                _ => Err(pest::error::Error::new_from_span(
                    pest::error::ErrorVariant::CustomError { message: "Failed to parse float".into() },
                    tuple_pair.as_span(),
                )),
            }
        }
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Failed to parse xy_coordinates".into() },
            tuple_pair.as_span(),
        )),
    }
}

/// process string text data not include quote(All string values in spef file are not quoted).
fn process_string(pair: Pair<Rule>) -> Result<String, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    match pair_clone.as_str().parse::<String>() {
        Ok(value) => Ok(value),
        Err(_) => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Failed to parse string".into() },
            pair_clone.as_span(),
        )),
    }
}

/// process connection direction enum
fn process_conn_dir_enum(pair: Pair<Rule>) -> Result<spef_data::ConnectionDirection, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    match pair.as_str() {
        "I" => Ok(spef_data::ConnectionDirection::INPUT),
        "O" => Ok(spef_data::ConnectionDirection::OUTPUT),
        "B" => Ok(spef_data::ConnectionDirection::INOUT),
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Failed to parse connection direction".into() },
            pair_clone.as_span(),
        )),
    }
}

/// process connection type enum
fn process_conn_type_enum(pair: Pair<Rule>) -> Result<spef_data::ConnectionType, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    match pair.as_str() {
        "*I" => Ok(spef_data::ConnectionType::INTERNAL),
        "*P" => Ok(spef_data::ConnectionType::EXTERNAL),
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Failed to parse connection type".into() },
            pair_clone.as_span(),
        )),
    }
}

/// process section entry
fn process_section_entry(pair: Pair<Rule>) -> Result<spef_data::SpefSectionEntry, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    let line_no = pair.line_col().0;

    let mut inner_rules = pair_clone.into_inner();

    let section_name_pair = inner_rules.next().unwrap();

    let section_name_result = process_string(section_name_pair);

    match section_name_result {
        Ok(result) => {
            let section_type: spef_data::SectionType = match result {
                s if s == "NAME_MAP" => spef_data::SectionType::NAMEMAP,
                s if s == "PORTS" => spef_data::SectionType::PORTS,
                s if s == "CONN" => spef_data::SectionType::CONN,
                s if s == "CAP" => spef_data::SectionType::CAP,
                s if s == "RES" => spef_data::SectionType::RES,
                s if s == "END" => spef_data::SectionType::END,
                _ => {
                    // 处理未知规则的情况
                    return Err(pest::error::Error::new_from_span(
                        pest::error::ErrorVariant::CustomError { message: "Unknown rule".into() },
                        pair.as_span(),
                    ));
                }
            };
            Ok(spef_data::SpefSectionEntry::new("tbd", line_no, section_type))
        },
        Err(_) => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Unknown rule".into() },
            pair.as_span(),
        )),
    }
}

/// process pest pairs that matches spef header section entry
fn process_header_entry(pair: Pair<Rule>) -> Result<spef_data::SpefHeaderEntry, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    let line_no = pair_clone.line_col().0;

    let mut inner_rules = pair_clone.into_inner();
    // println!("{inner_rules:#?}");

    // header_keyword_pair and header_value_pair are string pairs
    let header_keyword_pair = inner_rules.next().unwrap();
    let header_value_pair = inner_rules.next().unwrap();

    let keyword_pair_result = process_string(header_keyword_pair);
    let value_pair_result = process_string(header_value_pair);

    match (keyword_pair_result, value_pair_result) {
        (Ok(header_key), Ok(header_value)) => {
            Ok(spef_data::SpefHeaderEntry::new("tbd", line_no, header_key, header_value))
        }
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Unknown rule".into() },
            pair.as_span(),
        )),
    }
}

/// process pest pairs that matches spef namemap section entry
fn process_namemap_entry(pair: Pair<Rule>) -> Result<spef_data::SpefNameMapEntry, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    let line_no = pair_clone.line_col().0;

    let mut inner_rules = pair_clone.into_inner();
    // println!("{inner_rules:#?}");

    // name_index_pair is float pair, name_value_pair is string pair
    let name_index_pair = inner_rules.next().unwrap();
    let name_value_pair = inner_rules.next().unwrap();

    let index_pair_result = process_float(name_index_pair);
    let value_pair_result = process_string(name_value_pair);

    match (index_pair_result, value_pair_result) {
        (Ok(name_index), Ok(name_pair)) => {
            Ok(spef_data::SpefNameMapEntry::new("tbd", line_no, name_index as usize, &name_pair))
        }
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Unknown rule".into() },
            pair.as_span(),
        )),
    }
}

/// process pest pairs that matches spef ports section entry
fn process_port_entry(pair: Pair<Rule>) -> Result<spef_data::SpefPortEntry, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    let line_no = pair_clone.line_col().0;

    let mut inner_rules = pair_clone.into_inner();
    // println!("{inner_rules:#?}");

    // name_index_pair is float pair, name_value_pair is string pair
    let name_index_pair = inner_rules.next().unwrap();
    let conn_dir_pair = inner_rules.next().unwrap();
    let coordinates_pair = inner_rules.next().unwrap();

    let index_pair_result = process_float(name_index_pair);
    let dir_pair_result = process_conn_dir_enum(conn_dir_pair);
    let coor_pair_result = process_coordinates(coordinates_pair);

    match (index_pair_result, dir_pair_result, coor_pair_result) {
        (Ok(index), Ok(direction), Ok(coordinates)) => {
            Ok(spef_data::SpefPortEntry::new("tbd", line_no, index.to_string(), direction, coordinates))
        }
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Unknown rule".into() },
            pair.as_span(),
        )),
    }
}

/// process pest pairs that matches spef dnet section entry, creating a SpefNet
fn process_dnet_entry<'a>(
    pair: Pair<'a, Rule>,
    current_net: &'a mut spef_data::SpefNet,
) -> Result<spef_data::SpefNet, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    let line_no = pair_clone.line_col().0;

    let mut inner_rules = pair_clone.into_inner();

    let name_pair = inner_rules.next().unwrap();
    let cap_pair = inner_rules.next().unwrap();

    let name_pair_result = process_string(name_pair);
    let cap_pair_result = process_float(cap_pair);

    match (name_pair_result, cap_pair_result) {
        (Ok(name), Ok(cap)) => {
            current_net.name = name;
            current_net.line_no = line_no;
            current_net.lcap = cap;
            Ok(current_net.clone())
        }
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Unknown rule".into() },
            pair.as_span(),
        )),
    }
}

fn process_conn_entry(pair: Pair<Rule>) -> Result<spef_data::SpefConnEntry, pest::error::Error<Rule>> {
    let pair_clone = pair.clone();
    let line_no = pair_clone.line_col().0;

    let mut inner_rules = pair_clone.into_inner();

    let conn_type_pair = inner_rules.next().unwrap();
    let pin_name_pair = inner_rules.next().unwrap();
    let conn_dir_pair = inner_rules.next().unwrap();
    let coordinates_pair = inner_rules.next().unwrap();
    let load_pair = inner_rules.next().unwrap();
    let driver_pair = inner_rules.next().unwrap();

    let type_pair_result = process_conn_type_enum(conn_type_pair);
    let name_pair_result = process_string(pin_name_pair);
    let dir_pair_result = process_conn_dir_enum(conn_dir_pair);
    let coor_pair_result = process_coordinates(coordinates_pair);
    let load_pair_result = process_float(load_pair);
    let driver_pair_result = process_string(driver_pair);

    match (type_pair_result, name_pair_result, dir_pair_result, coor_pair_result, load_pair_result, driver_pair_result)
    {
        (Ok(conn_type), Ok(pin_name), Ok(conn_dir), Ok(coordinates), Ok(load), Ok(driver)) => {
            let current_conn =
                spef_data::SpefConnEntry::new("tbd", line_no, conn_type, conn_dir, pin_name, driver, load, coordinates);
            Ok(current_conn)
        }
        _ => Err(pest::error::Error::new_from_span(
            pest::error::ErrorVariant::CustomError { message: "Unknown rule".into() },
            pair.as_span(),
        )),
    }
}

pub fn parse_spef_file(spef_file_path: &str) -> Result<spef_data::SpefExchange, pest::error::Error<Rule>> {
    // !TODO: replace .expect with match or let if
    let unparsed_file = fs::read_to_string(spef_file_path).expect("cannot read file");
    let spef_entries = SpefParser::parse(Rule::file, &unparsed_file).expect("unsuccessful parse");

    let mut exchange_data =
        spef_data::SpefExchange::new(spef_data::SpefStringValue { value: spef_file_path.to_string() });

    let mut current_net: spef_data::SpefNet = spef_data::SpefNet::new(0, "None".to_string(), 0.0);

    for entry in spef_entries {
        match entry.as_rule() {
            Rule::header_entry => {
                let parse_result = process_header_entry(entry.clone());
                match parse_result {
                    Ok(result) => {
                        exchange_data.add_header_entry(result.clone());
                        result
                    }
                    Err(err) => return Err(err.clone()),
                };
            }
            Rule::name_map_entry => {
                let parse_result = process_namemap_entry(entry.clone());
                match parse_result {
                    Ok(result) => {
                        exchange_data.add_namemap_entry(result.clone());
                        result
                    }
                    Err(err) => return Err(err.clone()),
                };
            }
            Rule::ports_entry => {
                let parse_result = process_port_entry(entry.clone());
                match parse_result {
                    Ok(result) => {
                        exchange_data.add_port_entry(result.clone());
                        result
                    }
                    Err(err) => return Err(err.clone()),
                };
            }
            Rule::dnet_entry => {
                let parse_result = process_dnet_entry(entry, &mut current_net);
                match parse_result {
                    Ok(result) => result,
                    Err(err) => return Err(err.clone()),
                };
            }
            Rule::conn_entry => {
                let parse_result = process_conn_entry(entry);
                match parse_result {
                    Ok(result) => {
                        current_net.add_connection(&result);
                        result
                    }
                    Err(err) => return Err(err.clone()),
                };
            }
            Rule::cap_entry => {}
            Rule::res_entry => {}
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(exchange_data)
}
