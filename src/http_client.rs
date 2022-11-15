use serde::{Serialize, Deserialize};
use super::{credentials::Credentials, auth_response::AuthResponse};

pub struct AuthClient{
    url: String,
    credentials: Credentials,
}

pub struct HttpClient{
    url: String,
    token: String,
    body: String,

}

#[derive(Default)]
pub struct HttpClientBuilder{
    url: String,
    token: String,
    body: String,
}



impl HttpClient{
    pub fn new() -> HttpClientBuilder{
        HttpClientBuilder { 
            ..Default::default()
         }
    }

    pub fn send(&self) -> Result<(), String>{
        let client = reqwest::blocking::Client::new();
        let response = client.post(&self.url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token)
            .body(self.body.clone())
            .send().map_err(|e| e.to_string())?;
        
        if response.status().is_success() {
            return Ok(());
        }
        
        Err(response.text().map_err(|e| e.to_string())?)

    }
    
}

impl HttpClientBuilder {
    pub fn url(mut self, path: String) -> HttpClientBuilder {
        self.url = format!("{}{}", "https://trialeservices.yeka.gr/WebServicesApi/api/Documents/", path);
        self
    }

    pub fn token(mut self, token: String) -> HttpClientBuilder {
        self.token = token;
        self
    }

    pub fn body(mut self, body: String) -> HttpClientBuilder {
        self.body = body;
        self
    }
    pub fn build(self) -> HttpClient {
        HttpClient { url: self.url , token: self.token, body: self.body }
    }
}


impl AuthClient{
    pub fn new(user_name: String, password: String, user_type: String) -> Self {
        Self {
            url: "https://trialeservices.yeka.gr/WebServicesApi/api/Authentication".to_string(),
            credentials: Credentials { user_name, password, user_type }
        }
    }

    pub fn get_token(&self) -> Result<String, String> {

        let body = serde_json::to_string(&self.credentials).map_err(|e| e.to_string())?;
        let client = reqwest::blocking::Client::new();
        let response = client.post(&self.url)
            .header("Content-Type", "application/json")
            .body(body)
            .send().map_err(|e| e.to_string())?;
        
        let resp_body = response.text().map_err(|e| e.to_string())?;
        let auth_res = serde_json::from_str::<AuthResponse>(&resp_body).unwrap();
        Ok(auth_res.token)

    }
}

//Unit test

#[cfg(test)]
mod tests {
    use super::*;
    use crate::working_status::*;
    

    #[test]
    fn auth_request_test() {
        let auth_client = AuthClient::new("IKA000FD00E".to_string(), "51992450".to_string(), "01".to_string());

        let token = auth_client.get_token().unwrap();


        println!("{}", token);
        
    }
    #[test]
    fn full_request_test(){
        let auth_client = AuthClient::new("IKA000FD00E".to_string(), "51992450".to_string(), "01".to_string());

        let token = auth_client.get_token().unwrap();

        let wsc = WorkingStatusChangesWrapper{
            working_status_changes: WorkingStatusChanges { 
                working_status_change:vec![ WorkingStatusChange{
                    f_aa_pararthmatos: "0".to_string(),
                    f_rel_protocol: "Αρ. Πρωτοκόλλου".to_string(),
                    f_rel_date: "22/11/2022".to_string(),
                    f_comments: "No comment".to_string(),
                    ergazomenoi: Ergazomenoi { 
                        egrazomenos: vec![ Ergazomenos{
                            f_afm: "999999999".to_string(),
                            f_eponymo: "Επώνυμο".to_string(),
                            f_onoma: "Ονομα".to_string(),
                            f_date: "22/11/2022".to_string(),
                            f_working_time_digital_organization: "1".to_string(),
                            f_full_employment_hours: "40,0".to_string(),
                            f_week_days: "5".to_string(),
                            f_euelikto_wrario_minutes: "30".to_string(),
                            f_working_card: "1".to_string(),
                            f_dialeimma_minutes: "30".to_string(),
                            f_dialeimma_entos_wrariou: "1".to_string(),
                        }]
                    }
                }]
            } 
        };
        let body = serde_json::to_string(&wsc).unwrap();

        let path = "WKChgWK".to_string();
        let res = HttpClient::new()
            .url(path)
            .token(token)
            .body(body)
            .build()
            .send();
        

        match  res {
            Ok(()) => {
                println!("{}", "Επιτυχημένη αποστολή");
            }
            Err(e) => {
                println!("{}", e);
            }
            
        }

    }
}