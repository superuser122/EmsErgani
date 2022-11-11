use crate::{anaggelia::*, working_status::*, wto::*};


pub fn get_anaggelia(lines: Vec<&str>) -> Result<AnaggeliesE3Wrapper, String> {
    let mut anaggelies: Vec<AnaggeliaE3> = Vec::new();
    for line in lines{
        let row = line.clone().to_string();
        if row.starts_with("1") {
            let cells: Vec<&str> = row.split(";").collect();
            if cells.len() < 119 { return Err("Κάποια γραμμή αναγγελίας έχει μη αποδεκτό πλήθος κολόνων".to_string());}
            let anaggelia = AnaggeliaE3 {
                f_aa_pararthmatos: cells[1].to_string(),
                f_rel_protocol: cells[2].to_string(),
                f_rel_date: cells[3].to_string(),
                f_ypiresia_sepe: cells[4].to_string(),
                f_ypiresia_oaed: cells[5].to_string(),
                f_ergodotikh_organwsh: cells[6].to_string(),
                f_kad_kyria: cells[6].to_string(),
                f_kad_deyt_1: cells[7].to_string(),
                f_kad_deyt_2: cells[8].to_string(),
                f_kad_deyt_3: cells[9].to_string(),
                f_kad_deyt_4: cells[10].to_string(),
                f_kad_pararthmatos: cells[11].to_string(),
                f_kallikratis_pararthmatos: cells[12].to_string(),
                f_eponymo: cells[13].to_string(),
                f_onoma: cells[14].to_string(),
                f_eponymo_patros: cells[15].to_string(),
                f_onoma_patros: cells[16].to_string(),
                f_eponymo_mitros: cells[17].to_string(),
                f_onoma_mitros: cells[18].to_string(),
                f_topos_gennhshs: cells[19].to_string(),
                f_birthdate: cells[20].to_string(),
                f_sex: cells[21].to_string(),
                f_yphkoothta: cells[22].to_string(),
                f_typos_taytothtas: cells[23].to_string(),
                f_ar_taytothtas: cells[24].to_string(),
                f_ekdousa_arxh: cells[25].to_string(),
                f_date_ekdosis: cells[26].to_string(),
                f_date_ekdosis_lixi: cells[27].to_string(),
                f_res_permit_inst: cells[28].to_string(),
                f_res_permit_inst_type: cells[29].to_string(),
                f_res_permit_inst_ar: cells[30].to_string(),
                f_res_permit_inst_lixi: cells[31].to_string(),
                f_res_permit_ap: cells[32].to_string(),
                f_res_permit_ap_type: cells[33].to_string(),
                f_res_permit_ap_ar: cells[34].to_string(),
                f_res_permit_ap_lixi: cells[35].to_string(),
                f_res_permit_visa: cells[36].to_string(),
                f_res_permit_visa_ar: cells[37].to_string(),
                f_res_permit_visa_from: cells[38].to_string(),
                f_res_permit_visa_to: cells[39].to_string(),
                f_marital_status: cells[40].to_string(),
                f_arithmos_teknon: cells[41].to_string(),
                f_afm: cells[42].to_string(),
                f_doy: cells[43].to_string(),
                f_amika: cells[44].to_string(),
                f_amka: cells[45].to_string(),
                f_code_anergias:cells[46].to_string(),
                f_ar_vivliou_anilikou: cells[47].to_string(),
                f_dieythinsi: cells[48].to_string(),
                f_kallikratis: cells[49].to_string(),
                f_tk: cells[50].to_string(),
                f_til: cells[51].to_string(),
                f_fax: cells[52].to_string(),
                f_email: cells[53].to_string(),
                f_epipedo_morfosis: cells[54].to_string(),
                f_professional_education: cells[55].to_string(),
                f_expertise_field: cells[56].to_string(),
                f_subject_area: cells[57].to_string(),
                f_subject_group: cells[58].to_string(),
                f_education_agency: cells[59].to_string(),
                f_education_date_from: cells[60].to_string(),
                f_education_date_to: cells[61].to_string(),
                f_duration: cells[62].to_string(),
                f_education_year: cells[63].to_string(),
                f_fl1: cells[64].to_string(),
                f_fl2: cells[65].to_string(),
                f_fl3: cells[66].to_string(),
                f_fl4: cells[67].to_string(),
                f_pc: cells[68].to_string(),
                f_pc_other: cells[69].to_string(),
                f_proslipsidate: cells[70].to_string(),
                f_proslipsitime: cells[71].to_string(),
                f_apoxwrisitime: cells[72].to_string(),
                f_orario: cells[73].to_string(),
                f_wresexternal: cells[74].to_string(),
                f_week_hours: cells[75].to_string(),
                f_orariodialeima: cells[76].to_string(),
                f_eidikothta: cells[77].to_string(),
                f_proipiresia: cells[78].to_string(),
                f_apodoxes: cells[79].to_string(),
                f_hour_apodoxes: cells[80].to_string(),
                f_protiergasia: cells[81].to_string(),
                f_sxeshapasxolisis: cells[82].to_string(),
                f_orismenou_apo: cells[83].to_string(),
                f_orismenou_ews: cells[84].to_string(),
                f_kathestosapasxolisis: cells[85].to_string(),
                f_xaraktirismos: cells[86].to_string(),
                f_special_case: cells[87].to_string(),
                f_apoalliperioxi: cells[88].to_string(),
                f_nationalityalli: cells[89].to_string(),
                f_kallikratisalli: cells[90].to_string(),
                f_responsible_position: cells[91].to_string(),
                f_working_time_digital_organization: cells[92].to_string(),
                f_full_employment_hours: cells[93].to_string(),
                f_week_days: cells[94].to_string(),
                f_euelikto_wrario_minutes: cells[95].to_string(),
                f_working_card: cells[96].to_string(),
                f_dialeimma_minutes: cells[97].to_string(),
                f_dialeimma_entos_wrariou: cells[98].to_string(),
                f_topothetisiepistoli: cells[99].to_string(),
                f_topothetisioaed: cells[100].to_string(),
                f_programaoaed: cells[101].to_string(),
                f_replaceprograma: cells[102].to_string(),
                f_replaceprograma_afm: cells[103].to_string(),
                f_replaceprograma_amka: cells[104].to_string(),
                f_epidomaoaed: cells[105].to_string(),
                f_epidoma_ypiresia_oaed: cells[106].to_string(),
                f_sk_protocol: cells[107].to_string(),
                f_sk_date: cells[108].to_string(),
                f_comments: cells[109].to_string(),
                f_eponymo_idiotitas: cells[110].to_string(),
                f_onoma_idiotitas: cells[111].to_string(),
                f_idiotita_idiotitas: cells[112].to_string(),
                f_dieythinsi_idiotitas: cells[113].to_string(),
                f_afm_idiotitas: cells[114].to_string(),
                f_afm_proswpoy: cells[115].to_string(),
                f_file: cells[116].to_string(),
                f_foreign_file: cells[117].to_string(),
                f_young_file: cells[118].to_string(),
            };
            anaggelies.push(anaggelia);
        }
    }

    let anaggelies_wrapper = AnaggeliesE3Wrapper{
        anaggelies: AnaggeliesE3{
            anaggelia: anaggelies,
        }
    };
    Ok(anaggelies_wrapper)
}

pub fn get_working_status(lines: Vec<&str>) -> Result<WorkingStatusChangesWrapper, String> {

    let mut wsc = WorkingStatusChange {
        ..Default::default()
    };
    
    for line in lines{
        let row = line.clone().to_string();
        if row.starts_with("1") {
            let cells: Vec<&str> = row.split(";").collect();
            if cells.len() < 5 { return Err("Κάποια γραμμή αλλαγής εργασιακής σχέσης έχει μη αποδεκτό πλήθος κολόνων".to_string());}
            wsc.f_aa_pararthmatos = cells[1].to_string();
            wsc.f_rel_protocol = cells[2].to_string();
            wsc.f_rel_date = cells[3].to_string();
            wsc.f_comments = cells[4].to_string();
        }
        else if row.starts_with("2") {
            let cells: Vec<&str> = row.split(";").collect();
            if cells.len() < 12 { return Err("Κάποια γραμμή αλλαγής εργασιακής σχέσης έχει μη αποδεκτό πλήθος κολόνων".to_string());}
            let ergazomenos = crate::working_status::Ergazomenos {
                f_afm: cells[1].to_string(),
                f_eponymo: cells[2].to_string(),
                f_onoma: cells[3].to_string(),
                f_date: cells[4].to_string(), 
                f_working_time_digital_organization: cells[5].to_string(),
                f_full_employment_hours: cells[6].to_string(),
                f_week_days: cells[7].to_string(),
                f_euelikto_wrario_minutes: cells[8].to_string(),
                f_working_card:  cells[9].to_string(),
                f_dialeimma_minutes:  cells[10].to_string(),
                f_dialeimma_entos_wrariou:  cells[11].to_string(), 
            };
            wsc.ergazomenoi.egrazomenos.push(ergazomenos);
        }
    }

    let wscsw = WorkingStatusChangesWrapper{
        working_status_changes: WorkingStatusChanges {
             working_status_change: vec![wsc] 
        }
    };

    Ok(wscsw)
} 

pub fn get_wtos(lines: Vec<&str>) -> Result<WtosWrapper, String>{
    
    let mut wtosw = WtosWrapper{
        wtos: Wtos {
             wto: Vec::new() 
        }
    };
    
    for line in lines{
        let row = line.to_string();
        if row.starts_with("1") {
            let cells: Vec<&str> = row.split(";").collect();
            //TODO: cells size check
            let wto = Wto {
                f_aa_pararthmatos: cells[1].to_string(),
                f_rel_protocol: cells[2].to_string(),
                f_rel_date: cells[3].to_string(),
                f_comments: cells[4].to_string(),
                f_from_date: cells[5].to_string(),
                f_to_date: cells[6].to_string(),
                ergazomenoi: crate::wto::Ergazomenoi {
                    ergazomenoi_wto: Vec::new()
                }
            };
            wtosw.wtos.wto.push(wto);
        }
        else if row.starts_with("2") {
            let cells: Vec<&str> = row.split(";").collect();
            //TODO: cells size check
            let ergazomenos = ErgazomenoiWTO {
                f_afm: cells[1].to_string(),
                f_eponymo: cells[2].to_string(),
                f_onoma: cells[3].to_string(),
                f_day: if cells[4].is_empty() { None} else { Some(cells[4].to_string())},
                f_date: if cells[5].is_empty() { None} else { Some(cells[5].to_string())},
                egrazomenos_analytics: ErgazomenosAnalytics{
                    egrazomenos_wto_analytics: Vec::new()
                }
            };
            //Strong rule wto has ONE and only element 
            if wtosw.wtos.wto.len() == 0 { return Err("Προσπάθεια εισαγωγής γραμμής εργαζομένου σε άδειο παράρτημα".to_string());}
            wtosw.wtos.wto[0].ergazomenoi.ergazomenoi_wto.push(ergazomenos);
        }
        else if row.starts_with("3") {
            let cells: Vec<&str> = row.split(";").collect();
            if cells.len() < 4 { return Err("Κάποια γραμμή αναλυτικών εργαζομένου έχει μη αποδεκτό πλήθος κολόνων".to_string());}
            let erg_analytic = ErgazomenosWTOAnalytics {
                f_type: cells[1].to_string(),
                f_from: cells[2].to_string(),
                f_to: cells[3].to_string(), 
            };
            match wtosw.wtos.wto[0].ergazomenoi.ergazomenoi_wto.last_mut() {
                Some(last_erg) => {
                    last_erg.egrazomenos_analytics.egrazomenos_wto_analytics.push(erg_analytic);
                },
                None => {
                     return Err("Προσπάθεια εισαγωγής αναλυτικών εργαζομένου σε άδεια λίστα εργαζομένων".to_string());
                }
            } 
        }
    }

    Ok(wtosw)
}