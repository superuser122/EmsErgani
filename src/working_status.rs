use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkingStatusChangesWrapper{
    #[serde(rename = "WorkingStatusChanges")]
    pub working_status_changes: WorkingStatusChanges,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkingStatusChanges{
    #[serde(rename = "WorkingStatusChange")]
    pub working_status_change: Vec<WorkingStatusChange>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkingStatusChange{
    pub f_aa_pararthmatos: String,
    pub f_rel_protocol: String,
    pub f_rel_date: String,
    pub f_comments: String,
    #[serde(rename = "Egrazomenoi")]
    pub ergazomenoi: Ergazomenoi,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ergazomenoi{
    #[serde(rename = "Egrazomenos")]
    pub egrazomenos: Vec<Ergazomenos>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ergazomenos{
    pub f_afm: String,
    pub f_eponymo: String,
    pub f_onoma: String,
    pub f_date: String, 
    pub f_working_time_digital_organization: String,
    pub f_full_employment_hours: String,
    pub f_week_days: String,
    pub f_euelikto_wrario_minutes: String,
    pub f_working_card: String,
    pub f_dialeimma_minutes: String,
    pub f_dialeimma_entos_wrariou: String, 
}

//Unit testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_status() {
        let wsc = WorkingStatusChangesWrapper{
            working_status_changes: WorkingStatusChanges { 
                working_status_change:vec![ WorkingStatusChange{
                    f_aa_pararthmatos: "ΑΑ".to_string(),
                    f_rel_protocol: "Αρ. Πρωτοκόλλου".to_string(),
                    f_rel_date: "22/11/2022".to_string(),
                    f_comments: "No comment".to_string(),
                    ergazomenoi: Ergazomenoi { 
                        egrazomenos: vec![ Ergazomenos{
                            f_afm: "ΑΦΜ".to_string(),
                            f_eponymo: "Επώνυμο".to_string(),
                            f_onoma: "Ονομα".to_string(),
                            f_date: "22/11/2022".to_string(),
                            f_working_time_digital_organization: "1".to_string(),
                            f_full_employment_hours: "1".to_string(),
                            f_week_days: "40".to_string(),
                            f_euelikto_wrario_minutes: "30".to_string(),
                            f_working_card: "1".to_string(),
                            f_dialeimma_minutes: "30".to_string(),
                            f_dialeimma_entos_wrariou: "1".to_string(),
    
                        }]
                    }
            
                }]

            } 

        };
       
        let json_str = serde_json::to_string(&wsc).unwrap();

        println!("{}", json_str);


        //assert!(mongo_url.is_ok());
    }
}

