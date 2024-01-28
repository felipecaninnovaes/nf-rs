use chrono::NaiveDate;
use uuid::Uuid;

pub struct CreateIdAndCurrentDateModel {
    pub id: Uuid,
    pub current_date: NaiveDate,
}

pub fn create_id_and_current_date() -> CreateIdAndCurrentDateModel {
    let uuid = Uuid::new_v4();
    let current_date = chrono::Utc::now().naive_utc().date();
    CreateIdAndCurrentDateModel {
        id: uuid,
        current_date,
    }
}
