extern crate csv;

use std::error::Error;
use std::io;
//use std::process;
//use std::fs::File;
//use std::io::Prelude;

fn print_records() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let admin: AdminManager = AdminManager::new("admins.txt");
    println!("AdminManager instance admin filename: {}", admin.get_filename());

}

struct AdminManager {
    filename: String,
    admins: HashMap::new();
}
impl AdminManager {
    fn new(filename: &str) -> AdminManager {
        AdminManager {
            filename: filename.to_string(),
            admins: 
        }
    }
    fn get_filename(self) -> String {
        self.filename
    }
    fn get_clearance(&self, name: String) -> u8 {
        let admin_index: usize = self.admin_names.iter().position(|x: &String| x == &name).unwrap();
        self.admin_clearances[admin_index]
    }
    fn add_admin(&self, name: String, clearance: u8) -> None {
        admin_names.append
    }
}