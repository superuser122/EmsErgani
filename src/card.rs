use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CardWrapper{
    #[serde(rename = "Cards")]
    pub cards: Cards,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cards{
    #[serde(rename = "Card")]
    pub card: Vec<Card>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Card{
    pub f_afm_ergodoti: String,
    pub f_aa: String,
    pub f_comments: String,
    #[serde(rename = "Details")]
    pub details: Detail
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Detail{
    #[serde(rename = "CardDetails")]
    pub details: Vec<CardDetails>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CardDetails{
    pub f_afm: String,
    pub f_eponymo: String,
    pub f_onoma: String,
    pub f_type: String,
    pub f_reference_date: String,
    pub f_date: String,
    pub f_aitiologia: String,
}


//Unit test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_card() {

        let detail = CardDetails{
            f_afm: "9999999999".to_string(),
            f_eponymo: "Επώνυμο".to_string(),
            f_onoma: "Όνομα".to_string(),
            f_type: "1".to_string(),
            f_reference_date: "22/12/2022".to_string(),
            f_date: "22/11/2022".to_string(),
            f_aitiologia: "".to_string(),
        };

        let details = Detail{
            details: vec![detail],
        };

        let card = Card{
            f_afm_ergodoti: "888888888".to_string(),
            f_aa: "123".to_string(),
            f_comments: "Comment".to_string(),
            details: details,
        };

        let cards = Cards{
            card:  vec![card]
        };

        let card_w =  CardWrapper{
            cards: cards,
        };

       
   
        let json_str = serde_json::to_string(&card_w).unwrap();

        println!("{}", json_str);
        
    }
}