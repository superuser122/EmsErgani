use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OvertimesWrapper{
    overtimes: Overtimes,
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