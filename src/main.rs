// You will then create a class called Cell and assign the column as class attributes.
// Create getter and setter methods. The objects will be stored in 3 different data structures.
// You can use the data structures native to the language. You are also free to pick what these data
// structures are but one of them must be a HashMap or something similar.

// struct is class trait is interface sort of

use regex::Regex;
use serde::{Deserialize, Serialize, de::IntoDeserializer};
use std::{collections::HashMap, fs::File, io, process};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CellPhone {
    // csv::invalid option deserializes null to None!
    #[serde(deserialize_with = "csv::invalid_option")]
    oem: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    model: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    launch_announced: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    launch_status: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    body_dimensions: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    body_weight: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    body_sim: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    display_type: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    display_size: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    display_resolution: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    features_sensors: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    platform_os: Option<String>,
}

impl CellPhone {
    fn into_array(self) -> [Option<String>; 12] {
        [
            self.oem,
            self.model,
            self.launch_announced,
            self.launch_status,
            self.body_dimensions,
            self.body_weight,
            self.body_sim,
            self.display_type,
            self.display_size,
            self.display_resolution,
            self.features_sensors,
            self.platform_os,
        ]
    }

    fn get_oem(&self) -> &Option<String> {
        return &self.oem;
    }

    fn set_oem(&mut self, new_oem: Option<String>) {
        self.oem = new_oem;
    }

    fn get_model(&self) -> &Option<String> {
        return &self.model;
    }

    fn set_model(&mut self, new_model: Option<String>) {
        self.oem = new_model;
    }

    fn get_launched_announced(&self) -> &Option<String> {
        return &self.launch_announced;
    }

    fn set_launch_announced(&mut self, new_year: Option<String>) {
        self.launch_announced = new_year;
    }

    fn get_launched_status(&self) -> &Option<String> {
        return &self.launch_status;
    }

    fn set_launch_status(&mut self, new_year: Option<String>) {
        self.launch_status = new_year;
    }

    fn get_body_dimensions(&self) -> &Option<String> {
        return &self.body_dimensions;
    }

    fn set_body_dimensions(&mut self, new_dimensions: Option<String>) {
        self.body_dimensions = new_dimensions;
    }

    fn get_body_weight(&self) -> &Option<String> {
        return &self.body_weight;
    }

    fn set_body_weight(&mut self, new_weight: Option<String>) {
        self.body_weight = new_weight;
    }

    fn get_body_sim(&self) -> &Option<String> {
        return &self.body_sim;
    }

    fn set_body_sim(&mut self, new_sim: Option<String>) {
        self.body_sim = new_sim;
    }

    fn get_display_type(&self) -> &Option<String> {
        return &self.display_type;
    }

    fn set_display_type(&mut self, new_display_type: Option<String>) {
        self.body_sim = new_display_type;
    }

    fn get_display_size(&self) -> &Option<String> {
        return &self.display_size;
    }

    fn set_display_size(&mut self, new_size: Option<String>) {
        self.display_size = new_size;
    }

    fn get_display_resolution(&self) -> &Option<String> {
        return &self.display_resolution;
    }

    fn set_display_resolution(&mut self, new_resolution: Option<String>) {
        self.display_resolution = new_resolution;
    }

    fn get_features_sensors(&self) -> &Option<String> {
        return &self.features_sensors;
    }

    fn set_features_sensors(&mut self, new_features: Option<String>) {
        self.features_sensors = new_features;
    }

    fn get_platform_os(&self) -> &Option<String> {
        return &self.platform_os;
    }

    fn set_platform_os(&mut self, new_platform: Option<String>) {
        self.platform_os = new_platform;
    }

    fn sanitize_launch_announced(&mut self) {
        if self.launch_announced.is_some() {
            let captures: Vec<&str> = Regex::new(r"\d{4}")
                .unwrap()
                .find_iter(self.launch_announced.as_ref().unwrap())
                .map(|x| x.as_str())
                .collect();
            if captures.len() > 0 {
                self.set_launch_announced(Some(captures[0].to_string()));
            } else {
                self.set_launch_announced(None);
            }
        }
    }

    fn sanitize_launch_status(&mut self) {
        if self.launch_status.is_some() {
            let captures: Vec<&str> = Regex::new(r"\d{4}")
                .unwrap()
                .find_iter(self.get_launched_status().as_ref().unwrap())
                .map(|x| x.as_str())
                .collect();
            if captures.len() > 0 {
                self.set_launch_status(Some(captures[0].to_string()));
            } else {
                self.set_launch_status(None);
            }
        }
    }

    fn sanitize_dimensions(&mut self) {
        if self.body_dimensions.is_some() {
            let mut count = 0;
            for ch in self.body_dimensions.as_ref().unwrap().split("") {
                if ch.to_string() == "x" {
                    count += 1;
                }
            }

            if count < 2 {
                self.set_body_dimensions(None);
            }
        }
    }

    fn sanitize_body_weight(&mut self) {
        if self.body_weight.is_some() {
            let captures: Vec<&str> = Regex::new(r"[^-]\d*\.?\d*")
                .unwrap()
                .find_iter(self.body_weight.as_ref().unwrap())
                .map(|x| x.as_str())
                .collect();
            if captures.len() > 0 {
                self.set_body_weight(Some(captures[0].to_string()));
            } else {
                self.set_body_weight(None);
            }
        }
    }

    fn sanitize_body_sim(&mut self) {
        if self.body_sim.is_some() {
            let mut count = 0;
            for ch in self.body_sim.as_ref().unwrap().split("") {
                if ch == "x" {
                    count += 1;
                }
            }

            if count < 2 {
                self.set_body_sim(None);
            }
        }
    }

    fn sanitize_display_type(&mut self) {
        if self.body_sim.is_some() && self.body_sim.as_ref().unwrap() == "V1" {
            self.set_display_type(None);
        }
    }

    fn sanitize_display_size(&mut self) {
        if self.display_size.is_some() {
            let captures: Vec<&str> = Regex::new(r"\d*\.?\d+[^a-z|\s]")
                .unwrap()
                .find_iter(self.display_size.as_ref().unwrap())
                .map(|x| x.as_str())
                .collect();
            if captures.len() > 0 {
                self.set_display_size(Some(captures[0].to_string()));
            } else {
                self.set_display_size(None);
            }
        }
    }

    fn sanitize_display_resolution(&mut self) {
        if self.display_resolution.is_some() {
            // TODO: change this from regex to split string and just make sure it contains at least 3 x's to be valid
            let mut count = 0;
            for ch in self.display_resolution.as_ref().unwrap().split("") {
                if ch.to_string() == "x" {
                    count += 1;
                }
            }

            if count < 1 {
                self.set_body_dimensions(None);
            }
        }
    }

    fn sanitize_features_sensors(&mut self) {
        if self.features_sensors.is_some() && self.features_sensors.as_ref().unwrap() == "V1" {
            self.set_features_sensors(None);
        }
    }

    fn sanitize_platform_os(&mut self) {
        if self.platform_os.is_some() {
            let result: Vec<&str> = self.platform_os.as_ref().unwrap().split(",").collect();
            self.set_platform_os(Some(result[0].to_string()));
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct SanitizedCellPhone {
    oem: Option<String>,
    model: Option<String>,
    launch_announced: Option<String>,
    launch_status: Option<String>,
    body_dimensions: Option<String>,
    body_weight: Option<f64>,
    body_sim: Option<String>,
    display_type: Option<String>,
    display_size: Option<f64>,
    display_resolution: Option<String>,
    features_sensors: Option<String>,
    platform_os: Option<String>,
}

impl SanitizedCellPhone {
    fn get_oem(&self) -> &Option<String> {
        return &self.oem;
    }

    fn set_oem(&mut self, new_oem: Option<String>) {
        self.oem = new_oem;
    }

    fn get_model(&self) -> &Option<String> {
        return &self.model;
    }

    fn set_model(&mut self, new_model: Option<String>) {
        self.oem = new_model;
    }

    fn get_launched_announced(&self) -> &Option<String> {
        return &self.launch_announced;
    }

    fn set_launch_announced(&mut self, new_year: Option<String>) {
        self.launch_announced = new_year;
    }

    fn get_launched_status(&self) -> &Option<String> {
        return &self.launch_status;
    }

    fn set_launch_status(&mut self, new_year: Option<String>) {
        self.launch_status = new_year;
    }

    fn get_body_dimensions(&self) -> &Option<String> {
        return &self.body_dimensions;
    }

    fn set_body_dimensions(&mut self, new_dimensions: Option<String>) {
        self.body_dimensions = new_dimensions;
    }

    fn get_body_weight(&self) -> &Option<f64> {
        return &self.body_weight;
    }

    fn set_body_weight(&mut self, new_weight: Option<f64>) {
        self.body_weight = new_weight;
    }

    fn get_body_sim(&self) -> &Option<String> {
        return &self.body_sim;
    }

    fn set_body_sim(&mut self, new_sim: Option<String>) {
        self.body_sim = new_sim;
    }

    fn get_display_type(&self) -> &Option<String> {
        return &self.display_type;
    }

    fn set_display_type(&mut self, new_display_type: Option<String>) {
        self.body_sim = new_display_type;
    }

    fn get_display_size(&self) -> &Option<f64> {
        return &self.display_size;
    }

    fn set_display_size(&mut self, new_size: Option<f64>) {
        self.display_size = new_size;
    }

    fn get_display_resolution(&self) -> &Option<String> {
        return &self.display_resolution;
    }

    fn set_display_resolution(&mut self, new_resolution: Option<String>) {
        self.display_resolution = new_resolution;
    }

    fn get_features_sensors(&self) -> &Option<String> {
        return &self.features_sensors;
    }

    fn set_features_sensors(&mut self, new_features: Option<String>) {
        self.features_sensors = new_features;
    }

    fn get_platform_os(&self) -> &Option<String> {
        return &self.platform_os;
    }

    fn set_platform_os(&mut self, new_platform: Option<String>) {
        self.platform_os = new_platform;
    }
}

pub struct FileHandler {
    file_path: String,
    records: Vec<SanitizedCellPhone>,
    table: HashMap<String, SanitizedCellPhone>,
}

impl FileHandler {
    pub fn new(file_path: &str) -> FileHandler {
        FileHandler {
            file_path: file_path.to_string(),
            records: Vec::new(),
            table: HashMap::<String, SanitizedCellPhone>::new(),
        }
    }

    pub fn read_csv(&mut self) -> io::Result<()> {
        let file = File::open(&self.file_path)?; // question mark passes error if not found (syntactic sugar)
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut i = 0; // TODO: remove this before deployment
        for result in rdr.deserialize() {
            let mut record: CellPhone = result?;
            record.sanitize_launch_announced();
            record.sanitize_launch_status();
            record.sanitize_dimensions();
            record.sanitize_body_weight();
            record.sanitize_body_sim();
            record.sanitize_display_type();
            record.sanitize_display_size();
            record.sanitize_display_resolution();
            record.sanitize_features_sensors();
            record.sanitize_platform_os();
            // println!("{:?}", i);
            if i == 200 {
                println!("{:?}", record);
            }
            // println!("{:?}", record.body_weight);
            
            let clean_record = SanitizedCellPhone {
                oem: if record.oem.is_some() { record.get_oem().to_owned() } else { None },
                model: if record.model.is_some() { record.get_model().to_owned() } else { None },
                launch_announced: if record.launch_announced.is_some() { record.get_launched_announced().to_owned() } else { None },
                launch_status: if record.launch_status.is_some() { record.get_launched_status().to_owned() } else { None },
                body_dimensions: if record.body_dimensions.is_some() { record.get_body_dimensions().to_owned() } else { None },
                body_weight: if record.body_weight.is_some() {Some(record.get_body_weight().to_owned().unwrap().parse::<f64>().unwrap()) } else { None },
                body_sim: if record.body_sim.is_some() { record.get_body_sim().to_owned() } else { None },
                display_type: if record.display_type.is_some() { record.get_display_type().to_owned() } else { None },
                display_size: if record.display_size.is_some() {Some(record.get_display_size().to_owned().unwrap().parse::<f64>().unwrap()) } else { None },
                display_resolution: if record.display_resolution.is_some() { record.get_display_resolution().to_owned() } else { None },
                features_sensors: if record.display_resolution.is_some() { record.get_features_sensors().to_owned() } else { None },
                platform_os: if record.platform_os.is_some() { record.get_platform_os().to_owned() } else { None },
            };
            
            // composite key: oem-model
            let entry = self
                .table
                .entry(
                    format!(
                        "{}-{}",
                        clean_record.oem.as_ref().unwrap(),
                        clean_record.model.as_ref().unwrap()
                    )
                    .to_string(),
                )
                .or_insert_with(|| clean_record);
            i = i + 1;
        }

        for val in self.table.values() {
            if val.body_weight.is_some() && val.body_weight.unwrap() >= 453.6 {

                println!("{:?}", val);
            }
        }

        // self.records = self.table.values().cloned().collect();
        // 883 records so invalidated 1000 - 833 = 167 records
        // println!("{:?}", self.records.len());
        Ok(())
    }
}

// DONE BUT REVISIT: sanitize other columns DONE BUT DOUBLE CHECK LATER

// TODO NEXT: begin conversion of record structs to new structs

// TODO: implement stats on FinalCell
// TODO: implement Display
// like totals, max value, min value, median value, others? to save calculation time on averages

// TODO: need another data structure
// NOTE: the 3 different data structures are a HashMap, vec, and binary heap on max weight?
// DONE: hashmap, vec,

// ToString method that will convert your objects details to a string for printing.
// Calculate statistics on columns, for numeric, descriptive stats such as mean, median, standard deviation, etc. For categorical columns, perhaps Mode or count of unique values.
// Listing of unique values for each column.
// Ability to add an object and input data for each object's variable.
// Ability to delete an object.

// fn update_average(&mut self) {
//     let total: i32 = self.list.iter().sum();
//     self.average = total as f64 / self.list.len() as f64;
// }

fn main() -> io::Result<()> {
    let mut fh = FileHandler::new("src/cells.csv");

    if let Err(err) = fh.read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    Ok(())
}
