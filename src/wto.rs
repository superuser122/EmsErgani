use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WtosWrapper{
    #[serde(rename = "WTOS")]
    wtos: Wtos, 
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Wtos{
    #[serde(rename = "WTO")]
    wto: Vec<Wto>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Wto{
    pub f_aa_pararthmatos: String,
    pub f_rel_protocol: String,
    pub f_rel_date: String,
    pub f_comments: String,
    pub f_from_date: String,
    pub f_to_date: String,
    #[serde(rename = "Ergazomenoi")]
    pub ergazomenoi: Ergazomenoi,
}



#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ergazomenoi{
    #[serde(rename = "ErgazomenoiWTO")]
    ergazomenoi_wto: Vec<ErgazomenoiWTO>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ErgazomenoiWTO{
    pub f_afm: String,
    pub f_eponymo: String,
    pub f_onoma: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_date: Option<String>,
    #[serde(rename = "ErgazomenosAnalytics")]
    pub egrazomenos_analytics: ErgazomenosAnalytics,


}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ErgazomenoiWTOType{
    pub f_afm: String,
    pub f_eponymo: String,
    pub f_onoma: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_date: Option<String>,
    #[serde(rename = "ErgazomenosAnalytics")]
    pub egrazomenos_analytics: ErgazomenosAnalytics,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ErgazomenosAnalytics{
    #[serde(rename = "ErgazomenosWTOAnalytics")]
    pub egrazomenos_wto_analytics: Vec<ErgazomenosWTOAnalytics>,

}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ErgazomenosWTOAnalytics{
    pub f_type: String,
    pub f_from: String,
    pub f_to: String, 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_wto() {

       
        let wto_analytic1 = ErgazomenosWTOAnalytics{
            f_type: "ΑΝ".to_string(),
            f_from: "12/11/2022".to_string(),
            f_to: "12/11/2022".to_string(),
        };
        
        let wto_analytic2 = ErgazomenosWTOAnalytics{
            f_type: "ΑΝ".to_string(),
            f_from: "12/11/2022".to_string(),
            f_to: "12/11/2022".to_string(),
        };

        let analytics = vec![wto_analytic1, wto_analytic2];

        let erg_analytics = ErgazomenosAnalytics{
            egrazomenos_wto_analytics: analytics,
        };

        let ergazomenoi_wto = ErgazomenoiWTO{
            f_afm: "999999999".to_string(),
            f_eponymo: "Επωνυμο".to_string(),
            f_onoma: "Όνομα".to_string(), 
            f_day: Some("22/11/2022".to_string()),
            f_date: None,
            egrazomenos_analytics: erg_analytics, 
        };

        let ergazomenoi = Ergazomenoi{
            ergazomenoi_wto: vec![ergazomenoi_wto],
        };

        let wto = Wto{
            f_aa_pararthmatos: "Αρ. Παραρτήματος".to_string(),
            f_rel_protocol: "Αρ. Πρωτοκόλλου".to_string(),
            f_rel_date: "22/11/2022".to_string(),
            f_comments: "Σχόλιο".to_string(),
            f_from_date: "22/04/2022".to_string(),
            f_to_date: "".to_string(),
            ergazomenoi: ergazomenoi,
        };

        let wtos = Wtos{
            wto:vec![wto]
        };

        let wto_wrapper = WtosWrapper{
            wtos: wtos
        };

        let json_str = serde_json::to_string(&wto_wrapper).unwrap();

        println!("{}", json_str);


        
    }
}
