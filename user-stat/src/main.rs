use user_stat::pb::User;

fn main() {
    let user = User {
        name: "John".to_string(),
        email: "john@example.com".to_string(),
    };
    println!("{:?}", user);
}
