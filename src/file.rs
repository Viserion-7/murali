use crate::errors;

use std::fs::File;
use csv::ReaderBuilder;

/// Exists primarily to make function headers contain less arguments
pub struct MemberData {
    pub name: String,
    pub gender: String,
    pub roll_number: String
}

/**
 * @brief    Searches predefined CSV to find real name, roll number and gender of a person with
 *           their discord username as the key.
 *
 * @return   Ok<Some> if member data found 
 *           Ok<None> if 
 *           Err() if failed in execution 
 */
pub fn get_member_data(key: &str) -> Result<Option<MemberData>, errors::GetRecordError> 
{
    const RECORD_GET_EXPECT_MESSAGE: &str = "Members data must be set";

    let file = File::open("MemberData.csv")?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let csv_iter = rdr.records();
    for item in csv_iter {
        if let Ok(record) = item { 
            let user_name = record.get(0).expect(RECORD_GET_EXPECT_MESSAGE);

            if user_name == key {
                return Ok(Some(
                        MemberData {
                            name: record.get(1).expect(RECORD_GET_EXPECT_MESSAGE).to_owned(),
                            gender: record.get(3).expect(RECORD_GET_EXPECT_MESSAGE).to_owned(),
                            roll_number: record.get(2).expect(RECORD_GET_EXPECT_MESSAGE).to_owned()
                        }
                        ));
            }

        } else if let Err(e) = item {
            return Err(crate::errors::GetRecordError::CSVError(e));
        }
    };

    Ok(None)
}
