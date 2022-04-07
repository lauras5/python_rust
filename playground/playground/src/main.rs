use std::io;

fn main() {
    // user account instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // let user2 : User = build_user(String::from("someemail@example.com"), String::from("someusername345"));

    // entire instance must be mutable if we are changing. rust wont allow only certain fields
    user1.email = String::from("anotheremail@email.com");
    user1.update_count(5);
    user1.print_count();
    user1.print_email();
}

// user account + struct fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // uses initialization shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}

impl User {
    fn update_count(&mut self, count: u64) -> () {
        self.sign_in_count += count;
    }

    fn print_count(&self) -> () {
        println!("Sign in count for user {} is {}.", self.username, self.sign_in_count);
    }

    fn print_email(&self) -> () {
        println!("User {}'s email is {}.", self.username, self.email);
    }
}
