use core::cmp::Ordering;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io, process,
};

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
    pub fn new() -> CellPhone {
        CellPhone {
            oem: Some("".to_string()),
            model: Some("".to_string()),
            launch_announced: Some("".to_string()),
            launch_status: Some("".to_string()),
            body_dimensions: Some("".to_string()),
            body_weight: Some("".to_string()),
            body_sim: Some("".to_string()),
            display_type: Some("".to_string()),
            display_size: Some("".to_string()),
            display_resolution: Some("".to_string()),
            features_sensors: Some("".to_string()),
            platform_os: Some("".to_string()),
        }
    }

    fn get_oem(&self) -> &Option<String> {
        &self.oem
    }

    fn set_oem(&mut self, new_oem: Option<String>) {
        self.oem = new_oem;
    }

    fn get_model(&self) -> &Option<String> {
        &self.model
    }

    fn set_model(&mut self, new_model: Option<String>) {
        self.oem = new_model;
    }

    fn get_launched_announced(&self) -> &Option<String> {
        &self.launch_announced
    }

    fn set_launch_announced(&mut self, new_year: Option<String>) {
        self.launch_announced = new_year;
    }

    fn get_launched_status(&self) -> &Option<String> {
        &self.launch_status
    }

    fn set_launch_status(&mut self, new_year: Option<String>) {
        self.launch_status = new_year;
    }

    fn get_body_dimensions(&self) -> &Option<String> {
        &self.body_dimensions
    }

    fn set_body_dimensions(&mut self, new_dimensions: Option<String>) {
        self.body_dimensions = new_dimensions;
    }

    fn get_body_weight(&self) -> &Option<String> {
        &self.body_weight
    }

    fn set_body_weight(&mut self, new_weight: Option<String>) {
        self.body_weight = new_weight;
    }

    fn get_body_sim(&self) -> &Option<String> {
        &self.body_sim
    }

    fn set_body_sim(&mut self, new_sim: Option<String>) {
        self.body_sim = new_sim;
    }

    fn get_display_type(&self) -> &Option<String> {
        &self.display_type
    }

    fn set_display_type(&mut self, new_display_type: Option<String>) {
        self.body_sim = new_display_type;
    }

    fn get_display_size(&self) -> &Option<String> {
        &self.display_size
    }

    fn set_display_size(&mut self, new_size: Option<String>) {
        self.display_size = new_size;
    }

    fn get_display_resolution(&self) -> &Option<String> {
        &self.display_resolution
    }

    fn set_display_resolution(&mut self, new_resolution: Option<String>) {
        self.display_resolution = new_resolution;
    }

    fn get_features_sensors(&self) -> &Option<String> {
        &self.features_sensors
    }

    fn set_features_sensors(&mut self, new_features: Option<String>) {
        self.features_sensors = new_features;
    }

    fn get_platform_os(&self) -> &Option<String> {
        &self.platform_os
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
        if self.display_type.is_some() {
            if self.display_type.as_ref().unwrap() == "V1" {
                self.set_display_type(None);
            }

            if self.display_type.as_ref().unwrap() == "No" {
                self.set_display_type(None);
            }

            if self.display_type.as_ref().unwrap() == "-" {
                self.set_display_type(None);
            }
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
    id: Option<u64>,
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

impl Ord for SanitizedCellPhone {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.model).cmp(&(other.model))
    }
}

impl PartialOrd for SanitizedCellPhone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SanitizedCellPhone {
    fn eq(&self, other: &Self) -> bool {
        (self.model) == (other.model)
    }
}

impl Eq for SanitizedCellPhone {}

impl SanitizedCellPhone {
    pub fn new() -> SanitizedCellPhone {
        SanitizedCellPhone {
            id: Some(0),
            oem: Some("".to_string()),
            model: Some("".to_string()),
            launch_announced: Some("".to_string()),
            launch_status: Some("".to_string()),
            body_dimensions: Some("".to_string()),
            body_weight: Some(0.0),
            body_sim: Some("".to_string()),
            display_type: Some("".to_string()),
            display_size: Some(0.0),
            display_resolution: Some("".to_string()),
            features_sensors: Some("".to_string()),
            platform_os: Some("".to_string()),
        }
    }

    fn get_id(&self) -> &Option<u64> {
        &self.id
    }

    fn set_id(&mut self, id: Option<u64>) {
        self.id = id;
    }

    fn get_oem(&self) -> &Option<String> {
        &self.oem
    }

    fn set_oem(&mut self, new_oem: Option<String>) {
        self.oem = new_oem;
    }

    fn get_model(&self) -> &Option<String> {
        &self.model
    }

    fn set_model(&mut self, new_model: Option<String>) {
        self.model = new_model;
    }

    fn get_launched_announced(&self) -> &Option<String> {
        &self.launch_announced
    }

    fn set_launch_announced(&mut self, new_year: Option<String>) {
        self.launch_announced = new_year;
    }

    fn get_launched_status(&self) -> &Option<String> {
        &self.launch_status
    }

    fn set_launch_status(&mut self, new_year: Option<String>) {
        self.launch_status = new_year;
    }

    fn get_body_dimensions(&self) -> &Option<String> {
        &self.body_dimensions
    }

    fn set_body_dimensions(&mut self, new_dimensions: Option<String>) {
        self.body_dimensions = new_dimensions;
    }

    fn get_body_weight(&self) -> &Option<f64> {
        &self.body_weight
    }

    fn set_body_weight(&mut self, new_weight: Option<f64>) {
        self.body_weight = new_weight;
    }

    fn get_body_sim(&self) -> &Option<String> {
        &self.body_sim
    }

    fn set_body_sim(&mut self, new_sim: Option<String>) {
        self.body_sim = new_sim;
    }

    fn get_display_type(&self) -> &Option<String> {
        &self.display_type
    }

    fn set_display_type(&mut self, new_display_type: Option<String>) {
        self.body_sim = new_display_type;
    }

    fn get_display_size(&self) -> &Option<f64> {
        &self.display_size
    }

    fn set_display_size(&mut self, new_size: Option<f64>) {
        self.display_size = new_size;
    }

    fn get_display_resolution(&self) -> &Option<String> {
        &self.display_resolution
    }

    fn set_display_resolution(&mut self, new_resolution: Option<String>) {
        self.display_resolution = new_resolution;
    }

    fn get_features_sensors(&self) -> &Option<String> {
        &self.features_sensors
    }

    fn set_features_sensors(&mut self, new_features: Option<String>) {
        self.features_sensors = new_features;
    }

    fn get_platform_os(&self) -> &Option<String> {
        &self.platform_os
    }

    fn set_platform_os(&mut self, new_platform: Option<String>) {
        self.platform_os = new_platform;
    }

    fn display(&self) -> String {
        format!("Id: {0} Manufacturer: {1}, model: {2}, year announced: {3}, year launched: {4}, body dimensions: {5}, body weight: {6}, sim: {7}, display type: {8}, display size: {9}, resolution: {10}, features: {11}, platform: {12}",
        self.id.as_ref().unwrap(),
        self.oem.as_ref().unwrap(),
        self.model.as_ref().unwrap(),
        self.launch_announced.as_ref().unwrap(),
        self.launch_status.as_ref().unwrap(),
        self.body_dimensions.as_ref().unwrap(),
        self.body_weight.as_ref().unwrap(),
        self.body_sim.as_ref().unwrap(),
        self.display_type.as_ref().unwrap(),
        self.display_size.as_ref().unwrap(),
        self.display_resolution.as_ref().unwrap(),
        self.features_sensors.as_ref().unwrap(),
        self.platform_os.as_ref().unwrap())
    }
}

pub struct FileHandler {
    file_path: String,
    records: Vec<SanitizedCellPhone>,
    table: HashMap<String, Vec<SanitizedCellPhone>>,
    heap: BinaryHeap<SanitizedCellPhone>,
    total_weight: f64,
    max_weight: f64,
    min_weight: f64,
    max_display_size: f64,
    min_display_size: f64,
    total_display_size: f64,
    mean_weight: f64,
    mean_display_size: f64,
}

impl FileHandler {
    pub fn new(file_path: &str) -> FileHandler {
        FileHandler {
            file_path: file_path.to_string(),
            records: Vec::<SanitizedCellPhone>::new(),
            table: HashMap::<String, Vec<SanitizedCellPhone>>::new(),
            heap: BinaryHeap::<SanitizedCellPhone>::new(),
            total_weight: 0.0,
            max_weight: 0.0,
            min_weight: 0.0,
            max_display_size: 0.0,
            min_display_size: 0.0,
            total_display_size: 0.0,
            mean_weight: 0.0,
            mean_display_size: 0.0,
        }
    }

    pub fn read_csv(&mut self) -> io::Result<()> {
        let file = File::open(&self.file_path)?; // question mark passes error if not found (syntactic sugar)
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut i = 0;
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

            let clean_record = SanitizedCellPhone {
                id: Some(i),
                oem: if record.oem.is_some() {
                    record.get_oem().to_owned()
                } else {
                    None
                },
                model: if record.model.is_some() {
                    record.get_model().to_owned()
                } else {
                    None
                },
                launch_announced: if record.launch_announced.is_some() {
                    record.get_launched_announced().to_owned()
                } else {
                    None
                },
                launch_status: if record.launch_status.is_some() {
                    record.get_launched_status().to_owned()
                } else {
                    None
                },
                body_dimensions: if record.body_dimensions.is_some() {
                    record.get_body_dimensions().to_owned()
                } else {
                    None
                },
                body_weight: if record.body_weight.is_some() {
                    Some(
                        record
                            .get_body_weight()
                            .to_owned()
                            .unwrap()
                            .parse::<f64>()
                            .unwrap(),
                    )
                } else {
                    None
                },
                body_sim: if record.body_sim.is_some() {
                    record.get_body_sim().to_owned()
                } else {
                    None
                },
                display_type: if record.display_type.is_some() {
                    record.get_display_type().to_owned()
                } else {
                    None
                },
                display_size: if record.display_size.is_some() {
                    Some(
                        record
                            .get_display_size()
                            .to_owned()
                            .unwrap()
                            .parse::<f64>()
                            .unwrap(),
                    )
                } else {
                    None
                },
                display_resolution: if record.display_resolution.is_some() {
                    record.get_display_resolution().to_owned()
                } else {
                    None
                },
                features_sensors: if record.display_resolution.is_some() {
                    record.get_features_sensors().to_owned()
                } else {
                    None
                },
                platform_os: if record.platform_os.is_some() {
                    record.get_platform_os().to_owned()
                } else {
                    None
                },
            };

            self.add_clean_record(clean_record);
            i += 1;
        }

        Ok(())
    }

    fn add_clean_record(&mut self, rec: SanitizedCellPhone) {
        if rec.body_weight.is_some() {
            self.update_max_weight(rec.get_body_weight().unwrap());
            self.update_min_weight(rec.get_body_weight().unwrap());
            self.total_weight += rec.get_body_weight().unwrap();
        }

        if rec.display_size.is_some() {
            self.update_max_display_size(rec.get_display_size().unwrap());
            self.update_min_display_size(rec.get_display_size().unwrap());
            self.total_display_size += rec.get_display_size().unwrap();
        }
        // copy into vector
        self.records.push(rec.clone());
        // copy into heap
        self.heap.push(rec.clone());
        // insert into hashMap
        let rec_vec = self
            .table
            .entry(String::from(rec.get_oem().as_ref().unwrap()))
            .or_insert(Vec::<SanitizedCellPhone>::new());
        rec_vec.push(rec);
    }

    fn update_max_weight(&mut self, other: f64) {
        self.max_weight = f64::max(self.max_weight, other);
    }

    fn get_max_weight(&self) -> f64 {
        return self.max_weight;
    }

    fn update_min_weight(&mut self, other: f64) {
        self.min_weight = f64::min(self.min_weight, other);
    }

    fn get_min_weight(&self) -> f64 {
        return self.min_weight;
    }

    fn update_max_display_size(&mut self, other: f64) {
        self.max_display_size = f64::max(self.max_display_size, other);
    }

    fn update_min_display_size(&mut self, other: f64) {
        self.min_display_size = f64::min(self.min_display_size, other);
    }

    fn calc_mean_weight(&mut self) {
        if self.records.len() > 0 {
            self.mean_weight = self.total_weight / self.records.len() as f64;
        }
    }

    fn get_mean_weight(&self) -> f64 {
        return self.mean_weight;
    }

    fn calc_mean_display_size(&mut self) {
        if self.records.len() > 0 {
            self.mean_display_size = self.total_display_size / self.records.len() as f64;
        }
    }

    fn get_mean_display_size(&self) -> f64 {
        return self.mean_display_size;
    }

    fn get_median_weight(&mut self) -> f64 {
        if self.records.len() > 0 {
            self.records.sort_unstable();
            return self.records[self.records.len() / 2]
                .get_body_weight()
                .unwrap();
        }
        return 0.0;
    }

    fn get_median_display_size(&mut self) -> f64 {
        if self.records.len() > 0 {
            self.records.sort_unstable();
            return self.records[self.records.len() / 2]
                .get_display_size()
                .unwrap();
        }
        0.0
    }

    fn get_weight_std_dev(&self) -> Option<f64> {
        // Return standard deviation or None. First time using this type of Some or None Option function.
        // Help with algorithm from https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
        match (self.get_mean_weight(), self.records.len()) {
            (data_mean, count) if count > 0 => {
                let variance = self
                    .records
                    .iter()
                    .map(|value| {
                        let diff = data_mean - (*value.get_body_weight().as_ref().unwrap());

                        diff * diff
                    })
                    .sum::<f64>()
                    / count as f64;

                Some(variance.sqrt())
            }
            _ => None,
        }
    }

    fn get_display_size_std_dev(&self) -> Option<f64> {
        // Help from https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
        match (self.get_mean_display_size(), self.records.len()) {
            (data_mean, count) if count > 0 => {
                let variance = self
                    .records
                    .iter()
                    .map(|value| {
                        let diff = data_mean - (*value.get_display_size().as_ref().unwrap());

                        diff * diff
                    })
                    .sum::<f64>()
                    / count as f64;

                Some(variance.sqrt())
            }
            _ => None,
        }
    }

    fn delete_record(&mut self, record: SanitizedCellPhone) {
        // Delete record from all 3 data structures
        let mut found = false;
        let mut i = 0;
        // delete from HashMap
        let key = record.get_oem().as_ref().unwrap().to_string();
        let rec_vec = self.table.get_mut(&key).unwrap();

        while !found && i < rec_vec.len() {
            if rec_vec[i].get_id().unwrap() == record.get_id().unwrap() {
                rec_vec.remove(i);
                found = true;
            }
            i += 1;
        }

        found = false;
        let mut stack = Vec::<SanitizedCellPhone>::new();
        i = 0;
        // delete from heap
        while !found && i < self.heap.len() {
            let record = self.heap.pop().unwrap();
            if record.get_id().unwrap() == record.get_id().unwrap() {
                print!("{:?}", record.get_id());
                found = true;
            } else {
                stack.push(record);
            }
            i += 1;
        }

        while !stack.is_empty() {
            self.heap.push(stack.pop().unwrap());
        }

        // delete from records vector
        let record_id = record.get_id().unwrap();
        self.records.remove(record_id.try_into().unwrap());
    }

    fn get_unique_column_vals(&mut self, col_name: &str) -> HashSet<String> {
        let mut result = HashSet::<String>::new();
        for rec in self.records.iter() {
            match col_name {
                "oem" => {
                    if rec.oem.is_some() {
                        result.insert(rec.get_oem().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "model" => {
                    if rec.model.is_some() {
                        result.insert(rec.get_model().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "launch_announced" => {
                    if rec.launch_announced.is_some() {
                        result.insert(rec.get_launched_announced().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "launch_status" => {
                    if rec.launch_status.is_some() {
                        result.insert(rec.get_launched_status().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "body_dimensions" => {
                    if rec.body_dimensions.is_some() {
                        result.insert(rec.get_body_dimensions().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "body_sim" => {
                    if rec.body_sim.is_some() {
                        result.insert(rec.get_body_sim().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "display_type" => {
                    if rec.display_type.is_some() {
                        result.insert(rec.get_display_type().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "display_resolution" => {
                    if rec.display_resolution.is_some() {
                        result.insert(rec.get_display_resolution().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "features_sensors" => {
                    if rec.features_sensors.is_some() {
                        result.insert(rec.get_features_sensors().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                "platform_os" => {
                    if rec.platform_os.is_some() {
                        result.insert(rec.get_platform_os().clone().unwrap())
                    } else {
                        result.insert("".to_string())
                    }
                }
                _ => result.insert("column not found".to_string()),
            };
            if result.contains("column not found") {
                return result;
            };
        }
        // remove the empty string instances since they are actually nulls
        result.remove("");
        result
    }
}

fn main() -> io::Result<()> {
    let mut fh = FileHandler::new("src/cells.csv");
    // can read empty csv but it's a no op so no error needed.
    if let Err(err) = fh.read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    fh.calc_mean_weight();
    fh.calc_mean_display_size();
    println!(
        "----------------------------------------------------------------------------------------"
    );
    println!("What company (oem) has the highest average weight of the phone body?");

    let mut highest_mean_oem = String::from("");
    let mut highest_mean_weight = 0.0;
    for (oem, recs) in fh.table {
        let total_weight = recs
            .iter()
            .map(|rec| {
                if rec.body_weight.is_some() {
                    rec.get_body_weight().as_ref().unwrap()
                } else {
                    &0.0
                }
            })
            .sum::<f64>();
        let mean_weight = total_weight / recs.len() as f64;
        if mean_weight > highest_mean_weight {
            highest_mean_weight = mean_weight;
            highest_mean_oem = oem;
        }
    }

    println!("    {:?} with {:?}", highest_mean_oem, highest_mean_weight);
    println!(
        "----------------------------------------------------------------------------------------"
    );
    println!("Was there any phones that were announced in one year and released in another?");
    let mut announced_released_ne = Vec::<SanitizedCellPhone>::new();
    let mut launched_ge_2000 = Vec::<SanitizedCellPhone>::new();
    let mut only_one_feature = Vec::<SanitizedCellPhone>::new();

    for rec in fh.records.iter() {
        if rec.get_launched_status().is_some()
            && rec
                .get_launched_status()
                .as_ref()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                >= 2000
        {
            launched_ge_2000.push(rec.clone());
        }

        if rec.get_launched_announced().is_some() && rec.get_launched_status().is_some() {
            if rec.get_launched_announced() != rec.get_launched_status() {
                announced_released_ne.push(rec.clone());
            }
        }

        if rec.get_features_sensors().is_some() {
            let feature_count = rec.get_features_sensors().as_ref().unwrap().split(",");
            if feature_count.count() < 2 {
                only_one_feature.push(rec.clone());
            }
        }
    }

    for rec in announced_released_ne {
        println!(
            "    Oem: {:?} Model: {:?} Announced: {:?} Released: {:?}",
            rec.get_oem().as_ref().unwrap(),
            rec.get_model().as_ref().unwrap(),
            rec.get_launched_announced().as_ref().unwrap(),
            rec.get_launched_status().as_ref().unwrap()
        );
    }
    println!(
        "----------------------------------------------------------------------------------------"
    );

    println!("How many phones have only one feature sensor?");
    println!("{:?}", only_one_feature.len());
    println!(
        "----------------------------------------------------------------------------------------"
    );
    println!("What year had the most phones launched in the 2000s?");
    let mut year_count = HashMap::<String, i32>::new();
    for rec in launched_ge_2000 {
        let count = year_count
            .entry(rec.get_launched_status().as_ref().unwrap().to_string())
            .or_insert(0);
        *count += 1;
    }
    let mut max_year_count = 0;
    let mut max_year = "".to_string();
    for (year, count) in year_count {
        if count > max_year_count {
            max_year_count = count;
            max_year = year;
        }
    }
    println!(
        "    Most phones launched in {:?} with {:?} phones.",
        max_year, max_year_count
    );
    println!(
        "----------------------------------------------------------------------------------------"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{CellPhone, FileHandler, SanitizedCellPhone};
    use std::{ops::Deref, vec};

    #[test]
    fn test_dash_converted_to_null() {
        let mut test_cell_record = CellPhone {
            oem: Some("pirate".to_string()),
            model: Some("wooden leg".to_string()),
            launch_announced: Some("-2019".to_string()),
            launch_status: Some("-2019".to_string()),
            body_dimensions: Some("-2019".to_string()),
            body_weight: Some("-".to_string()),
            body_sim: Some("-2019".to_string()),
            display_type: Some("-2019".to_string()),
            display_size: Some("-2019".to_string()),
            display_resolution: Some("-2019".to_string()),
            features_sensors: Some("-2019".to_string()),
            platform_os: Some("-2019".to_string()),
        };

        test_cell_record.sanitize_body_weight();
        let result = Some(test_cell_record.get_body_weight());
        assert_eq!(Some(None).as_ref(), result);
    }

    #[test]
    fn get_avg_weight_even() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");

        for (i, val) in vec![2.0, 4.0, 6.0, 8.0].iter().enumerate() {
            let mut test_sani_rec = SanitizedCellPhone::new();
            test_sani_rec.set_oem(Some("test_oem".to_string()));
            test_sani_rec.set_model(Some("test_model".to_string()));
            test_sani_rec.set_body_weight(Some(*val.deref()));
            test_fh.add_clean_record(test_sani_rec);
        }

        test_fh.calc_mean_weight();
        let result = test_fh.get_mean_weight();
        assert_eq!(5.0, result);
    }

    #[test]
    fn get_avg_weight_odd() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");

        for (i, val) in vec![2.0, 4.0, 6.0].iter().enumerate() {
            let mut test_sani_rec = SanitizedCellPhone::new();
            test_sani_rec.set_oem(Some("test_oem".to_string()));
            test_sani_rec.set_model(Some("test_model".to_string()));
            test_sani_rec.set_body_weight(Some(*val.deref()));
            test_fh.add_clean_record(test_sani_rec);
        }

        test_fh.calc_mean_weight();
        let result = test_fh.get_mean_weight();
        assert_eq!(4.0, result);
    }

    #[test]
    fn get_avg_weight_no_records() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");
        test_fh.calc_mean_weight();
        let result = test_fh.get_mean_weight();
        assert_eq!(0.0, result);
    }

    #[test]
    fn display_san_rec() {
        let mut record = SanitizedCellPhone::new();
        record.set_oem(Some("test_oem".to_string()));
        record.set_model(Some("test_model".to_string()));
        let display_output = record.display();
        let expected = "Id: 0 Manufacturer: test_oem, model: test_model, year announced: , year launched: , body dimensions: , body weight: 0, sim: , display type: , display size: 0, resolution: , features: , platform: ";
        assert_eq!(expected, display_output);
    }

    #[test]
    fn unique_columns() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");

        for i in 0..4 {
            let mut test_sani_rec = SanitizedCellPhone::new();
            test_sani_rec.set_oem(Some("test_oem".to_string()));
            test_sani_rec.set_model(Some(format!("{}_test_model", i)));
            test_fh.add_clean_record(test_sani_rec);
        }
        let actual_man_set: std::collections::HashSet<String> =
            test_fh.get_unique_column_vals("oem");
        let actual_model_set: std::collections::HashSet<String> =
            test_fh.get_unique_column_vals("model");
        assert_eq!(1, actual_man_set.len());
        assert_eq!(4, actual_model_set.len());
    }

    #[test]
    fn unique_columns_column_not_found() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");

        for i in 0..4 {
            let mut test_sani_rec = SanitizedCellPhone::new();
            test_sani_rec.set_oem(Some("chicken".to_string()));
            test_fh.add_clean_record(test_sani_rec);
        }
        let actual: std::collections::HashSet<String> = test_fh.get_unique_column_vals("chicken");
        assert_eq!(1, actual.len());
        assert_eq!(true, actual.contains("column not found"));
    }

    #[test]
    fn delete_record_from_all_data_structures() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");

        for i in 0..4 {
            let mut test_sani_rec = SanitizedCellPhone::new();
            test_sani_rec.set_id(Some(i));
            test_sani_rec.set_oem(Some("test_oem".to_string()));
            test_sani_rec.set_model(Some(format!("{}_test_model", i)));
            test_fh.add_clean_record(test_sani_rec);
        }

        assert_eq!(4, test_fh.records.len());
        assert_eq!(4, test_fh.table["test_oem"].len());
        assert_eq!(4, test_fh.heap.len());

        test_fh.delete_record(test_fh.table["test_oem"][1].clone());

        assert_eq!(3, test_fh.records.len());
        assert_eq!(3, test_fh.table["test_oem"].len());
        assert_eq!(3, test_fh.heap.len());
    }

    #[test]
    fn calc_weight_std_dev() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");

        for i in 0..4 {
            let mut test_sani_rec = SanitizedCellPhone::new();
            test_sani_rec.set_id(Some(i));
            test_sani_rec.set_body_weight(Some(i as f64));
            test_fh.add_clean_record(test_sani_rec);
        }

        let actual = test_fh.get_weight_std_dev();
        assert_ne!(None, actual);
    }

    #[test]
    fn calc_display_std_dev() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");

        for i in 0..4 {
            let mut test_sani_rec = SanitizedCellPhone::new();
            test_sani_rec.set_id(Some(i));
            test_sani_rec.set_display_size(Some(i as f64));
            test_fh.add_clean_record(test_sani_rec);
        }

        let actual = test_fh.get_display_size_std_dev();
        assert_ne!(None, actual);
    }
}
