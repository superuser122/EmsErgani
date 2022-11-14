use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OvertimesWrapper{
    pub overtimes: Overtimes,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Overtimes{
    #[serde(rename = "Overtime")]
    pub overtime: Vec<Overtime>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Overtime{
    pub f_aa_pararthmatos: String,
    pub f_rel_protocol: String,
    pub f_rel_date: String,
    pub f_ypiresia_sepe: String,
    pub f_ergodotikh_organwsh: String,
    pub f_kad_kyria: String,
    pub f_kad_deyt_1: String,
    pub f_kad_deyt_2: String,
    pub f_kad_deyt_3: String,
    pub f_kad_deyt_4: String,
    pub f_kad_pararthmatos: String,
    pub f_kallikratis_pararthmatos: String,
    pub f_comments: String,
    pub f_afm_proswpoy: String,
    #[serde(rename = "Ergazomenoi")]
    pub ergazomenoi: Ergazomenoi,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ergazomenoi{
    #[serde(rename = "OvertimeErgazomenosDate")]
    pub ergazomenoi: Vec<Egrazomenos>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Egrazomenos{
    pub f_afm: String,
    pub f_amka: String,
    pub f_eponymo: String,
    pub f_onoma: String,
    pub f_date: String,
    pub f_from: String,
    pub f_to: String,
    pub f_cancellation: String,
    pub f_step: String,
    pub f_reason: String,
    pub f_weekdates: String,
    pub f_asee: String,
}


//Unit test 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_overtime() {
        let mut overtimes = OvertimesWrapper{
            ..Default::default()
        };

        let overtime = Overtime{
            f_aa_pararthmatos: "ΑΑ παραρτ.".to_string(),
            f_rel_protocol: "Αρ. πρωτοκ.".to_string(),
            f_rel_date: "12/12/2022".to_string(),
            f_ypiresia_sepe: "υπερ. σεπε".to_string(),
            f_ergodotikh_organwsh: "εργ. οργαν.".to_string(),
            f_kad_kyria: "Καδ κυρια".to_string(),
            f_kad_deyt_1: "καδ δευτερ. 1".to_string(),
            f_kad_deyt_2: "".to_string(),
            f_kad_deyt_3: "".to_string(),
            f_kad_deyt_4: "".to_string(),
            f_kad_pararthmatos: "καδ παραρτ.".to_string(),
            f_kallikratis_pararthmatos: "1205889".to_string(),
            f_comments: "σχόλιο".to_string(),
            f_afm_proswpoy: "9999988877".to_string(),
            ergazomenoi: crate::overtimes::Ergazomenoi {
                ergazomenoi: Vec::new()
            }
        };

        overtimes.overtimes.overtime.push(overtime);

        let ergazomenos = crate::overtimes::Egrazomenos{
            f_afm: "999999999".to_string(),
            f_amka: "αμκα".to_string(),
            f_eponymo: "Επώνυμο".to_string(),
            f_onoma: "Όνομα".to_string(),
            f_date: "12/12/2022".to_string(),
            f_from: "12/12/2022".to_string(),
            f_to: "12/12/2022".to_string(),
            f_cancellation: "0".to_string(),
            f_step: "3".to_string(),
            f_reason: "5".to_string(),
            f_weekdates: "0".to_string(),
            f_asee: "0".to_string(),
        };

        overtimes.overtimes.overtime[0].ergazomenoi.ergazomenoi.push(ergazomenos);
        let json_str = serde_json::to_string(&overtimes).unwrap();
        println!("{}", json_str);
    }
}
