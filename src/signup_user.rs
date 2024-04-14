use crate::models::User;
use rand::rngs::OsRng;
use std::error::Error;
use std::io::{self};
use regex::Regex;

use csv::{Reader, Writer};
use std::fs::OpenOptions;
use std::path::Path;

use argon2::{
    password_hash::{
        PasswordHasher, SaltString
    },
    Argon2
};



fn validate_date_of_birth(dob: &str) -> Result<(), String> {
    let regex = Regex::new(r"^(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01])-\d{4}$").unwrap();
    if regex.is_match(dob) {
        Ok(())
    } else {
        Err("Invalid date of birth format.".into())
    }
}

fn validate_name(name: &str) -> Result<(), String> {
    let regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();
    if regex.is_match(name) {
        Ok(())
    } else {
        Err("Invalid name format.".into())
    }
}

fn validate_national_id(id_number: &str) -> Result<(), String> {
    let regex = Regex::new(r"^\d{3}-\d{3}-\d{3}$").unwrap();
    if regex.is_match(id_number) {
        Ok(())
    } else {
        Err("Invalid ID number format.".into())
    }
}

fn user_exists(id_number: &str) -> Result<bool, Box<dyn Error>> {
    if !Path::new("users.csv").exists() {
        return Ok(false);
    }

    let mut rdr = Reader::from_path("users.csv")?;
    for result in rdr.records() {
        let record = result?;
        let id_number_match = record.get(2).map_or(false, |r| r == id_number);

        if id_number_match {
            return Ok(true);
        }
    }
    Ok(false)
}


fn save_user(user: &User) -> Result<(), Box<dyn Error>> {
    let file_path = "users.csv";
    let file_exists = Path::new(file_path).exists();
    let file = OpenOptions::new().append(true).create(true).open("users.csv")?;

    let mut wtr = Writer::from_writer(file);
  
    // If the file did not previously exist, write the header
    if !file_exists {
        wtr.write_record(&["name","date_of_birth","national_id", "password", "has_registered", "has_voted"])?;
    }
    wtr.write_record(&[
      &user.name,
      &user.date_of_birth,
      &user.national_id,
      &user.password,
      &user.has_voted.to_string(),
      &user.has_registered.to_string(),
      
    ])?;
  
    wtr.flush()?;
    println!("{} has been created successfully.", user.name);
    Ok(())
}

pub fn signup_user() -> Result<User, Box<dyn Error>> {
  let mut password = String::new();
  let mut name = String::new();
  let mut date_of_birth = String::new();
  let mut national_id = String::new();

  // Loop for name input
  loop {
      println!("Enter name:");
      name.clear(); // Clear previous input
      io::stdin().read_line(&mut name)?;
      name = name.trim().to_string();

      if validate_name(&name).is_ok() {
          break; // Exit loop if valid
      } else {
          println!("Invalid name format. Name should only contain letters and spaces.");
      }
  }

  // Loop for date of birth input
  loop {
      println!("Enter date of birth (MM-DD-YYYY):");
      date_of_birth.clear(); // Clear previous input
      io::stdin().read_line(&mut date_of_birth)?;
      date_of_birth = date_of_birth.trim().to_string();

      if validate_date_of_birth(&date_of_birth).is_ok() {
          break; // Exit loop if valid
      } else {
          println!("Invalid date of birth format. Please use MM-DD-YYYY format.");
      }
  }

  // Loop for ID number input
  loop {
      println!("Enter personal ID (xxx-xxx-xxx):");
      national_id.clear(); // Clear previous input
      io::stdin().read_line(&mut national_id)?;
      national_id = national_id.trim().to_string();

      if validate_national_id(&national_id).is_ok() {
          break; // Exit loop if valid
      } else {
          println!("Invalid ID number format. Expected format: xxx-xxx-xxx, where x is a number from 0-9.");
      }
  }


  println!("Enter password:");
  io::stdin().read_line(&mut password)?;

  name = name.trim().to_string();
  date_of_birth = date_of_birth.trim().to_string();
  national_id = national_id.trim().to_string();

  if user_exists(&national_id)? {
      return Err("National ID already exists. Voter Already Signed Up".into());
  }

    // Generate a salt using SaltString
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = match argon2.hash_password(password.trim().as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => return Err(e.to_string().into()), // Convert the error to a String and then into a Box<dyn Error>
    };

    let user = User::new(name, date_of_birth, national_id, password_hash);

    // Call save_user to save the newly created user
    save_user(&user)?;

    // Return the user instance after saving
    Ok(user)
}
