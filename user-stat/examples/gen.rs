use chrono::{DateTime, Days, Utc};
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::Name;
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Dummy, Deserialize, Serialize)]
struct UserStats {
    #[dummy(faker = "SafeEmail()")]
    email: String,
    #[dummy(faker = "Name()")]
    name: String,
    #[dummy(faker = "Word()")]
    gender: String,
    #[dummy(faker = "DateTimeBetween(
        start(365*5),
        end()
    )")]
    created_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(
        start(365*3),
        end()
    )")]
    last_visited_at: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(
        start(30),
        end()
    )")]
    last_watched_at: DateTime<Utc>,
    recent_watched: Vec<i32>,
    viewed_but_not_started: Vec<i32>,
    started_but_not_finished: Vec<i32>,
    finished: Vec<i32>,
    #[dummy(faker = "DateTimeBetween(
        start(30),
        end()
    )")]
    last_email_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(
        start(20),
        end()
    )")]
    last_in_app_notification: DateTime<Utc>,
    #[dummy(faker = "DateTimeBetween(
        start(10),
        end()
    )")]
    last_sms_notification: DateTime<Utc>,
}

fn main() {
    let user_stats = Faker.fake::<UserStats>();
    println!("{:?}", user_stats);
}

fn start(days: u64) -> DateTime<Utc> {
    DateTime::from(Utc::now())
        .checked_sub_days(Days::new(days))
        .unwrap()
}

fn end() -> DateTime<Utc> {
    DateTime::from(Utc::now())
}
