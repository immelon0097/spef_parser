use std::fmt::Debug;

pub trait SpefValue: Debug {
    fn is_string(&self) -> bool {
        false
    }
    fn is_float(&self) -> bool {
        false
    }
    fn is_name(&self) -> bool {
        false
    }
    fn is_coordinates(&self) -> bool {
        false
    }

    fn get_float_value(&self) -> f64 {
        panic!("This is unknown value.");
    }
    fn get_str_value(&self) -> &str {
        panic!("This is unknown value.");
    }
    fn get_name_string(&self) -> String {
        panic!("This is unknown value.");
    }
    fn get_coordinates(&self) -> (f64, f64) {
        panic!("This is unknown value.");
    }
}

/// spef float value.
/// # Examples
/// 1.7460
#[derive(Clone, Debug)]
pub struct SpefFloatValue {
    pub(crate) value: f64,
}

impl SpefValue for SpefFloatValue {
    fn is_float(&self) -> bool {
        true
    }

    fn get_float_value(&self) -> f64 {
        self.value
    }
}

/// spef string value.
/// # Examples
/// "sky130_fd_sc_hd__dfxtp_2"
#[derive(Clone, Debug)]
pub struct SpefStringValue {
    pub(crate) value: String,
}

impl SpefValue for SpefStringValue {
    fn is_string(&self) -> bool {
        true
    }

    fn get_str_value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug)]
pub struct SpefCoordinatesValue {
    pub(crate) value: (f64, f64),
}

impl SpefValue for SpefCoordinatesValue {
    fn is_coordinates(&self) -> bool {
        true
    }

    fn get_coordinates(&self) -> (f64, f64) {
        self.value
    }
}
/// spef line entry trait
pub trait SpefEntryTrait: Debug {
    fn is_section_entry(&self) -> bool {
        false
    }
    fn is_header_entry(&self) -> bool {
        false
    }
    fn is_namemap_entry(&self) -> bool {
        false
    }
    fn is_port_entry(&self) -> bool {
        false
    }
    fn is_dnet_entry(&self) -> bool {
        false
    }
    fn is_conn_entry(&self) -> bool {
        false
    }
    fn is_cap_entry(&self) -> bool {
        false
    }
    fn is_res_entry(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn std::any::Any;
}

/// spef line entry basic info and it's methods
#[derive(Clone, Debug)]
pub struct SpefEntryBasicInfo {
    file_name: SpefStringValue,
    line_no: usize,
}

impl SpefEntryBasicInfo {
    fn new(file_name: &str, line_no: usize) -> SpefEntryBasicInfo {
        SpefEntryBasicInfo { file_name: SpefStringValue { value: file_name.to_string() }, line_no: line_no }
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name.get_str_value()
    }

    pub fn get_line_no(&self) -> usize {
        self.line_no
    }
}

#[derive(Clone, Debug)]
pub enum SectionType {
    HEADER,
    PORTS,
    NAMEMAP,
    CONN,
    CAP,
    RES,
    END
}

#[derive(Clone, Debug)]
pub struct SpefSectionEntry {
    basic_info: SpefEntryBasicInfo,
    section_type: SectionType
}

impl SpefSectionEntry {
    pub fn new(file_name: &str, line_no: usize, section_type: SectionType) -> SpefSectionEntry {
        SpefSectionEntry { basic_info: SpefEntryBasicInfo::new(file_name, line_no), section_type }
    }

    pub fn get_basic_info(&self) -> &SpefEntryBasicInfo {
        &self.basic_info
    }

    pub fn get_section_type(&self) -> &SectionType {
        &self.section_type
    }
}

impl SpefEntryTrait for SpefSectionEntry {
    fn is_section_entry(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct SpefHeaderEntry {
    basic_info: SpefEntryBasicInfo,
    header_key: SpefStringValue,
    header_value: SpefStringValue,
}

impl SpefHeaderEntry {
    pub fn new(file_name: &str, line_no: usize, header_key: String, header_value: String) -> SpefHeaderEntry {
        SpefHeaderEntry { 
            basic_info: SpefEntryBasicInfo::new(file_name, line_no), 
            header_key: SpefStringValue { value: header_key }, 
            header_value: SpefStringValue { value: header_value } 
        }
    }

    pub fn get_basic_info(&self) -> &SpefEntryBasicInfo {
        &self.basic_info
    }

    pub fn get_header_key(&self) -> &str {
        &self.header_key.get_str_value()
    }
    
    pub fn get_header_value(&self) -> &str {
        &self.header_value.get_str_value()
    }
}

impl SpefEntryTrait for SpefHeaderEntry {
    fn is_header_entry(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Store each line of NameMap section
/// namemap entry example: *43353 us21\/_1057_
/// index: 43353
/// name: us21\/_1057_
#[derive(Clone, Debug)]
pub struct SpefNameMapEntry {
    basic_info: SpefEntryBasicInfo,
    index: usize,
    name: String,
}

impl SpefNameMapEntry {
    pub fn new(file_name: &str, line_no: usize, index: usize, name: &str) -> SpefNameMapEntry {
        SpefNameMapEntry { basic_info: SpefEntryBasicInfo::new(file_name, line_no), index, name: name.to_string() }
    }

    pub fn get_basic_info(&self) -> &SpefEntryBasicInfo {
        &self.basic_info
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
    
    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }
}

impl SpefEntryTrait for SpefNameMapEntry {
    fn is_namemap_entry(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Store each line of Port section
/// Port entry example: *37 I *C 633.84 0.242
/// name: "37"
/// direction: ConnectionType::INPUT
/// coordinates: (633.84, 0.242)
#[derive(Clone, Debug)]
pub enum ConnectionDirection {
    INPUT,
    OUTPUT,
    INOUT
}

#[derive(Clone, Debug)]
pub struct SpefPortEntry {
    basic_info: SpefEntryBasicInfo,
    name: String,
    direction: ConnectionDirection,
    coordinates: (f64, f64),
}

impl SpefPortEntry {
    pub fn new(file_name: &str, line_no: usize, name: String, direction: ConnectionDirection, coordinates: (f64, f64)) -> SpefPortEntry {
        SpefPortEntry { basic_info: SpefEntryBasicInfo::new(file_name, line_no), name, direction, coordinates }
    }

    pub fn get_basic_info(&self) -> &SpefEntryBasicInfo {
        &self.basic_info
    }
    
    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn get_direction(&self) -> &ConnectionDirection {
        &self.direction
    }

    pub fn get_coordinates(&self) -> (f64, f64) {
        self.coordinates
    }
}

impl SpefEntryTrait for SpefPortEntry {
    fn is_port_entry(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Store each line of Conn section
/// Conn entry example: *I *33272:Q O *C 635.66 405.835 *L 0 *D sky130_fd_sc_hd__dfxtp_1
/// name: "37"
/// direction: ConnectionType::INPUT
/// coordinates: (633.84, 0.242)
/// driving_cell: "sky130_fd_sc_hd__dfxtp_1"
#[derive(Clone, Debug)]
pub enum ConnectionType
{
  INTERNAL,
  EXTERNAL
}

#[derive(Clone, Debug)]
pub struct SpefConnEntry {
    basic_info: SpefEntryBasicInfo,
    pub conn_type: ConnectionType,
    pub conn_direction: ConnectionDirection,
    pub name: String,
    pub driving_cell: String,
    pub load: f64,
    pub layer: usize,

    pub coordinates: (f64, f64),
    pub ll_coordinate: (f64, f64),
    pub ur_coordinate: (f64, f64),
}

impl SpefConnEntry {
    pub fn new(
        file_name: &str, 
        line_no: usize, 
        conn_type: ConnectionType,
        conn_direction: ConnectionDirection,
        name: String,
        driving_cell: String,
        load: f64,
        coordinates: (f64, f64)) -> SpefConnEntry {
        SpefConnEntry { 
            basic_info: SpefEntryBasicInfo::new(file_name, line_no), 
            conn_type, 
            conn_direction,
            name, 
            driving_cell, 
            load, 
            layer: 0, 
            coordinates, 
            ll_coordinate: (0.0, 0.0), 
            ur_coordinate: (0.0, 0.0) }
    }

    pub fn get_basic_info(&self) -> &SpefEntryBasicInfo {
        &self.basic_info
    }
    
    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn get_conn_direction(&self) -> &ConnectionDirection {
        &self.conn_direction
    }

    pub fn get_conn_type(&self) -> &ConnectionType {
        &self.conn_type
    }
    
    pub fn get_coordinates(&self) -> (f64, f64) {
        self.coordinates
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.layer = layer;
    }
    pub fn set_ll_corr(&mut self, coordinates: (f64, f64)) {
        self.ll_coordinate = coordinates;
    }
    pub fn set_ur_corr(&mut self, coordinates: (f64, f64)) {
        self.ur_coordinate = coordinates;
    }
}

impl SpefEntryTrait for SpefConnEntry {
    fn is_conn_entry(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Store everthing about a net
/// Conn entry example: 3 *1:2 0.000520945
/// name: "1:2"
/// direction: ConnectionType::INPUT
/// coordinates: (633.84, 0.242)
/// driving_cell: "sky130_fd_sc_hd__dfxtp_1"

#[derive(Clone, Debug, Default)]
pub struct SpefNet {
    pub name: String,
    pub line_no: usize,
    pub lcap: f64,
    connection: Vec<SpefConnEntry>,
    caps: Vec<(String, String, f64)>,
    ress: Vec<(String, String, f64)>,
}

impl SpefNet {
    pub fn new(
        line_no: usize,
        name: String,
        lcap: f64,) -> SpefNet {
        SpefNet { name, line_no, lcap, connection: Vec::new(), caps: Vec::new(), ress: Vec::new() }
    }

    pub fn add_connection(&mut self, conn: &SpefConnEntry) {
        self.connection.push(conn.clone());
    }

    pub fn add_cap(&mut self, cap: (String, String, f64)) {
        self.caps.push(cap);
    }

    pub fn add_res(&mut self, res: (String, String, f64)) {
        self.ress.push(res);
    }
}

#[derive(Clone, Debug)]
/// Spef Exchange data structure with cpp
pub struct SpefExchange {
    file_name: SpefStringValue,
    header: Vec<SpefHeaderEntry>,
    namemap: Vec<SpefNameMapEntry>,
    ports: Vec<SpefPortEntry>,
    nets: Vec<SpefNet>
}

impl SpefExchange {
    pub fn new(
        file_name: SpefStringValue,
    ) -> SpefExchange {
        SpefExchange { file_name, header: Vec::new(), namemap: Vec::new(), ports: Vec::new(), nets: Vec::new() }
    }

    pub fn add_header_entry(&mut self, header: SpefHeaderEntry) {
        self.header.push(header);
    }
    pub fn add_namemap_entry(&mut self, namemap_entry: SpefNameMapEntry) {
        self.namemap.push(namemap_entry);
    }

    pub fn add_port_entry(&mut self, port_entry: SpefPortEntry) {
        self.ports.push(port_entry);
    }
    
    pub fn add_net(&mut self, net: SpefNet) {
        self.nets.push(net);
    }
}

#[derive(Clone, Debug)]
pub enum SpefParserData {
    // String(SpefStringValue),
    // Float(SpefFloatValue),
    SectionEntry(SpefSectionEntry),
    HeaderEntry(SpefHeaderEntry),
    NameMapEntry(SpefNameMapEntry),
    PortEntry(SpefPortEntry),
    ConnEntry(SpefConnEntry),
    NetEntry(SpefNet),
    Exchange(SpefExchange)
}
