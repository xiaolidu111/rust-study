struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}
fn generate_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sing_in_count: 10,
    }
}
fn main() {
    let mut user = User {
        active: true,
        username: String::from("fangxianwei"),
        email: String::from("xiaol@163.com"),
        sing_in_count: 20,
    };
    user.username = String::from("xiaolidu");
    let user1 = generate_user(String::from("xiao"), String::from("username"));
    let user2 = User { ..user1 };
    println!("{},{},{}", user.username, user1.active, user2.sing_in_count);
}
