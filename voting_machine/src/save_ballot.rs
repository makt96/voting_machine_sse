use crate::models::ElectionBallot;
use std::error::Error;
use std::fs::OpenOptions;
use csv;

pub fn save_election_ballot_to_csv(ballot: &ElectionBallot) -> Result<(), Box<dyn Error>> {
    let file_path = "ballot.csv";
    // The rest of the implementation follows, adjusting `self` to `ballot`.

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut file_is_empty = false;
    let metadata = file.metadata()?;
    if metadata.len() == 0 {
        file_is_empty = true;
    }

    let mut writer = csv::WriterBuilder::new()
        .has_headers(file_is_empty)
        .from_writer(file);

    for office in &ballot.offices { // Adjusted from self to ballot
        for candidate in &office.candidates {
            writer.serialize((
                &ballot.election_name, // Adjusted from self to ballot
                &office.office_name,
                &candidate.name,
                &candidate.political_party,
            ))?;
        }
    }
    writer.flush()?;
    Ok(())
}


//Test Modules

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Candidate, Office, ElectionBallot};

    #[test]
    fn test_save_election_ballot_to_csv() {
        let candidate = Candidate {
            name: "Candidate Name".to_string(),
            political_party: "Party".to_string(),
        };
        let office = Office {
            office_name: "Office Name".to_string(),
            candidates: vec![candidate],
        };
        let ballot = ElectionBallot {
            election_name: "Election Test".to_string(),
            offices: vec![office],
        };

        let result = save_election_ballot_to_csv(&ballot);
        assert!(result.is_ok());
        // Cleanup after test
        std::fs::remove_file("ballot.csv").unwrap();
    }
}
