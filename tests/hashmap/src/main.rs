use std::{collections::HashMap, hash::Hash};

fn main() {
    // Admin hashmap implementation
    let mut admins_hashmap: HashMap<String, u8> = HashMap::new();

    // Add new admin
    admins_hashmap.insert(
        "voxel".to_string(),
        0
    );

    // Query hashmap for admin names
    let admin_queries: Vec<&str> = vec!["voxel", "intruder"];
    for &query in &admin_queries {
        match admins_hashmap.get(query) {
            Some(clearance) => println!("Admin {} found! Clearance: {}", query, clearance),
            None => println!("Admin {} not found!", query)
        }
    }

    // Admin Struct implementation
    let mut admins: Vec<Admin> = vec![];

    // Add new admin
    admins.push(Admin::new("voxel", 0));
    for &query in &admin_queries {
        match Admin::is_admin(query, &admins) {
            true => println!("{} is an admin", query),
            false => println!("{} is NOT an admin", query)
        }
    }
    println!("voxel is admin: {}", Admin::is_admin("voxel", &admins));

}

struct Admin {
    name: String,
    clearance: u8
}

impl Admin {
    fn new(name: &str, clearance: u8) -> Admin {
        Admin {
            name: name.to_string(),
            clearance: clearance
        }
    }
    fn get_name(self) -> String {
        self.name
    }
    fn is_admin(name: &str, admins: &Vec<Admin>) -> bool {
        let strname: String = name.to_string();
        for admin in admins {
            if strname == admin.get_name() {
                return true
            }
        }
        false
    }
}
