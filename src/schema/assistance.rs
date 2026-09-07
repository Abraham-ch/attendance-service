use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum Status{
    Present,
    Absent,
    Late,
    Excused
}

pub struct Assistance{
    pub id: Uuid,
    pub student_id: Uuid,
    pub date: DateTime<Utc>, //date and created_at cannot represent the same, a student can be excused from a different day that the date of assistance belongs
    pub status: Status,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc> //an assistance can be updated in case a student were excused
}
