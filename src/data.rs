use std::error::Error;
use csv::Reader;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Participant {
    pub f_id: String,
    pub part1_country: String,
    pub anxietyrec1: Option<f32>,
    pub anxietyrec2: Option<f32>,
    pub anxietyrec3: Option<f32>,
    pub anxietyrec4: Option<f32>,
    pub anxietyrec5: Option<f32>,
    pub anxietyrec6: Option<f32>,
    pub anxietyrec7: Option<f32>,
    pub depression_rec1: Option<f32>,
    pub depression_rec2: Option<f32>,
    pub depression_rec3: Option<f32>,
    pub depression_rec4: Option<f32>,
    pub depression_rec5: Option<f32>,
    pub depression_rec6: Option<f32>,
    pub depression_rec7: Option<f32>,
    pub depression_rec8: Option<f32>,
    pub depression_rec9: Option<f32>,
    pub women_age: Option<u32>,
    pub number_of_pregnancy: Option<u32>,
    pub number_of_abortions: Option<u32>,
    pub bmi_pregnancy: Option<f32>,
    pub bmi_postpartum: Option<f32>
}

impl Participant {
    pub fn anxietyrec1(&self) -> Option<f32> {
        self.anxietyrec1
    }

    pub fn depression_rec1(&self) -> Option<f32> {
        self.depression_rec1
    }

    pub fn women_age(&self) -> Option<u32> {
        self.women_age
    }
}

pub fn load_data(filepath: &str) -> Result<Vec<Participant>, Box<dyn Error>> {
    let mut rdr = Reader::from_path(filepath)?;
    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let record: Participant = result?;
        data.push(record);
    }
    Ok(data)
}

impl Default for Participant {
    fn default() -> Self {
        Participant {
            f_id: String::new(),
            part1_country: String::new(),
            anxietyrec1: None,
            anxietyrec2: None,
            anxietyrec3: None,
            anxietyrec4: None,
            anxietyrec5: None,
            anxietyrec6: None,
            anxietyrec7: None,
            depression_rec1: None,
            depression_rec2: None,
            depression_rec3: None,
            depression_rec4: None,
            depression_rec5: None,
            depression_rec6: None,
            depression_rec7: None,
            depression_rec8: None,
            depression_rec9: None,
            women_age: None,
            number_of_pregnancy: None,
            number_of_abortions: None,
            bmi_pregnancy: None,
            bmi_postpartum: None
        }
    }
}