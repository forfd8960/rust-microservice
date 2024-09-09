use chrono::{DateTime, Days, Utc};
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::{Dummy, Fake, Faker};
use nanoid::nanoid;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Dummy, Deserialize, Serialize)]
enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Clone, Dummy, Deserialize, Serialize)]
struct UserStats {
    #[dummy(faker = "UniqueEmail")]
    email: String,
    #[dummy(faker = "Name()")]
    name: String,
    gender: Gender,
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
    #[dummy(faker = "IntList(10, 1, 100)")]
    recent_watched: Vec<i32>,
    #[dummy(faker = "IntList(10, 1, 100)")]
    viewed_but_not_started: Vec<i32>,
    #[dummy(faker = "IntList(10, 1, 100)")]
    started_but_not_finished: Vec<i32>,
    #[dummy(faker = "IntList(10, 1, 100)")]
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

struct IntList(pub i32, pub i32, pub i32); // does not handle locale, see locales module for more

impl Dummy<IntList> for Vec<i32> {
    fn dummy_with_rng<R: Rng + ?Sized>(v: &IntList, rng: &mut R) -> Vec<i32> {
        let (max, start, len) = (v.0, v.1, v.2);
        let size = rng.gen_range(0..max);
        (0..size)
            .map(|_| rng.gen_range(start..start + len))
            .collect()
        // const NAMES: &[&str] = &["John Doe", "Jane Doe"];
        // NAMES.choose(rng).unwrap()
    }
}

struct UniqueEmail; // does not handle locale, see locales module for more

impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UniqueEmail, rng: &mut R) -> String {
        // const NAMES: &[&str] = &["John Doe", "Jane Doe"];
        // NAMES.choose(rng).unwrap()
        let alphabet: [char; 16] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let id = nanoid!(10, &alphabet);
        let email: String = SafeEmail().fake_with_rng(rng);
        let at = email.find('@').unwrap();
        format!("{}.{}@{}", &email[..at], id, &email[at..])
    }
}
