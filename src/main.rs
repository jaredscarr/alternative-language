use regex::Regex;
use core::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, BinaryHeap, HashSet}, fs::File, io, process};

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

impl Eq for SanitizedCellPhone { }

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
        return &self.id;
    }

    fn set_id(&mut self, id: Option<u64>) {
        self.id = id;
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
        self.model = new_model;
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

    fn display(&self) -> String {
        return format!("Id: {0} Manufacturer: {1}, model: {2}, year announced: {3}, year launched: {4}, body dimensions: {5}, body weight: {6}, sim: {7}, display type: {8}, display size: {9}, resolution: {10}, features: {11}, platform: {12}",
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
        self.platform_os.as_ref().unwrap());
    } 
}

pub struct FileHandler {
    file_path: String,
    records: Vec<SanitizedCellPhone>,
    table: HashMap<String, SanitizedCellPhone>,
    heap: BinaryHeap<SanitizedCellPhone>,
    total_weight: f64,
    max_weight: f64,
    min_weight: f64,
    max_display_size: f64,
    min_display_size: f64,
    total_display_size: f64,
    avg_weight: f64,
    avg_display_size: f64
}

impl FileHandler {
    pub fn new(file_path: &str) -> FileHandler {
        FileHandler {
            file_path: file_path.to_string(),
            // 3 data structures chosen were a vector (array), binary heap, and a hashMap as per requirements
            records: Vec::<SanitizedCellPhone>::new(),
            table: HashMap::<String, SanitizedCellPhone>::new(),
            heap: BinaryHeap::<SanitizedCellPhone>::new(),
            total_weight: 0.0,
            max_weight: 0.0,
            min_weight: 0.0,
            max_display_size: 0.0,
            min_display_size: 0.0,
            total_display_size: 0.0,
            avg_weight: 0.0,
            avg_display_size: 0.0
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
            
            self.add_clean_record(i, clean_record);
            i += 1;
        }

        Ok(())
    }

    fn add_clean_record(&mut self, key: u64, rec: SanitizedCellPhone) {
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
        self.table.insert(key.to_string(), rec);
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

    fn calc_avg_weight(&mut self) {
        if self.records.len() > 0 {
            self.avg_weight = self.total_weight / self.records.len() as f64;
        }
    }

    fn get_avg_weight(&self) -> f64 {
        return self.avg_weight;
    }

    fn calc_avg_display_size(&mut self) {
        if self.records.len() > 0 {
            self.avg_display_size = self.total_display_size / self.records.len() as f64;
        }
    }

    fn get_avg_display_size(&self) -> f64 {
        return self.avg_display_size;
    }

    fn get_median_weight(&mut self) -> f64 {
        if self.records.len() > 0 {
            self.records.sort_unstable();
            return self.records[self.records.len() / 2].get_body_weight().unwrap();
        }
        return 0.0;
    }

    fn get_median_display_size(&mut self) -> f64 {
        if self.records.len() > 0 {
            self.records.sort_unstable();
            return self.records[self.records.len() / 2].get_display_size().unwrap();
        }
        return 0.0;
    }

    // fn get_weight_std_dev(&mut self) -> f64 {

    // }

    // fn get_display_size_std_dev(&mut self) -> f64 {

    // }

    fn delete_record(&mut self, record_id: u64) {
        // Delete record from all 3 data structures
        self.table.remove(&record_id.to_string());
        self.records.remove(record_id.try_into().unwrap());
        let mut found = false;
        let mut stack = Vec::<SanitizedCellPhone>::new();
        let mut i = 0;
        while !found && i < self.heap.len() {
            let record = self.heap.pop().unwrap();
            if record.get_id().unwrap() == record_id {
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
    }

    fn get_unique_column_vals(&mut self, col_name: &str) -> HashSet<String> {
        let mut result = HashSet::<String>::new();
        for (_k, v) in self.table.iter() {
            match col_name {
                "oem" => if v.oem.is_some() { result.insert(v.get_oem().clone().unwrap()) } else { result.insert("".to_string()) },
                "model" => if v .model.is_some() { result.insert(v.get_model().clone().unwrap()) } else { result.insert("".to_string()) },
                "launch_announced" => if v.launch_announced.is_some() { result.insert(v.get_launched_announced().clone().unwrap()) } else { result.insert("".to_string()) },
                "launch_status" => if v .launch_status.is_some() { result.insert(v.get_launched_status().clone().unwrap()) } else { result.insert("".to_string()) },
                "body_dimensions" => if v.body_dimensions.is_some() { result.insert(v.get_body_dimensions().clone().unwrap()) } else { result.insert("".to_string()) },
                "body_sim" => if v.body_sim.is_some() { result.insert(v.get_body_sim().clone().unwrap()) } else { result.insert("".to_string()) },
                "display_type" => if v.display_type.is_some() { result.insert(v.get_display_type().clone().unwrap()) } else { result.insert("".to_string()) },
                "display_resolution" => if v.display_resolution.is_some() { result.insert(v.get_display_resolution().clone().unwrap()) } else { result.insert("".to_string()) },
                "features_sensors" => if v.features_sensors.is_some() { result.insert(v.get_features_sensors().clone().unwrap()) } else { result.insert("".to_string()) },
                "platform_os" => if v.platform_os.is_some() { result.insert(v.get_platform_os().clone().unwrap()) } else { result.insert("".to_string()) },
                _ => result.insert("column not found".to_string()),
            };
            if result.contains("column not found") {
                return result;
            };
        }
        // remove the empty string instances since they are actually nulls
        result.remove("");
        return result;
    }
}

// TODO NEXT: implement stats on FileHandler MODE, standard deviation
// Calculate statistics on columns, for numeric, descriptive stats such as  standard deviation, etc. For categorical columns, perhaps Mode or count of unique values.
// Listing of unique values for each column.

// display everthying from requirements in the main function

// REPORT NOTES:
// I still need to understand the Result type and how to handle OK() Error() inside of functions


fn main() -> io::Result<()> {
    let mut fh = FileHandler::new("src/cells.csv");
    // can read empty csv but it's a no op so no error needed.
    if let Err(err) = fh.read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }

    // TODO: Display everything with clean prints here running all the stats
    fh.calc_avg_weight();
    let avg_weight = fh.get_avg_weight();
    let median_weight = fh.get_median_weight();
    println!("{:?}", avg_weight);
    println!("{:?}", median_weight);

    let unique_models = fh.get_unique_column_vals("model");
    println!("{:?}", unique_models.len());

    fh.delete_record(3);
    Ok(())
}


#[cfg(test)]
mod tests {
    use std::{vec, ops::Deref};
    use crate::{CellPhone, FileHandler, SanitizedCellPhone};

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
            test_fh.add_clean_record(i.try_into().unwrap(), test_sani_rec);
        }
        
        test_fh.calc_avg_weight();
        let result = test_fh.get_avg_weight();
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
            test_fh.add_clean_record(i.try_into().unwrap(), test_sani_rec);
        }

        test_fh.calc_avg_weight();
        let result = test_fh.get_avg_weight();
        assert_eq!(4.0, result);
    }

    #[test]
    fn get_avg_weight_no_records() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");
        test_fh.calc_avg_weight();
        let result = test_fh.get_avg_weight();
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
            test_fh.add_clean_record(i.try_into().unwrap(), test_sani_rec);
        }
        let actual_man_set: std::collections::HashSet<String> = test_fh.get_unique_column_vals("oem");
        let actual_model_set: std::collections::HashSet<String> = test_fh.get_unique_column_vals("model");
        assert_eq!(1, actual_man_set.len());
        assert_eq!(4, actual_model_set.len());
    }

    #[test]
    fn unique_columns_column_not_found() {
        let mut test_fh = FileHandler::new("src/empty_cells.csv");

        for i in 0..4 {
            let mut test_sani_rec = SanitizedCellPhone::new();
            test_sani_rec.set_oem(Some("chicken".to_string()));
            test_fh.add_clean_record(i.try_into().unwrap(), test_sani_rec);
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
            test_fh.add_clean_record(i.try_into().unwrap(), test_sani_rec);
        }

        assert_eq!(4, test_fh.records.len());
        assert_eq!(4, test_fh.table.len());
        assert_eq!(4, test_fh.heap.len());

        test_fh.delete_record(2);

        assert_eq!(3, test_fh.records.len());
        assert_eq!(3, test_fh.table.len());
        assert_eq!(3, test_fh.heap.len());
    }
}