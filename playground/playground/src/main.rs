// use std::io;

fn main() {
    let mut users: Vec<User> = Vec::new(); // vector of users
    // user account instance
    let mut user1: User = build_user(String::from("someone@example.com"), String::from("someusername123"));
    let user2: User = build_user(String::from("anotherone@email.com"), String::from("sonename345"));
    // entire instance must be mutable if we are changing. rust wont allow only certain fields
    user1.email = String::from("anotheremail@email.com");
    user1.update_count(5);
    user1.print_count();
    user1.print_email();

    users.push(user1);
    users.push(user2);
    print_users(&users);

    let base_rate: f32 = 0.03;
    // closure ex.
    let calculate_interest = |loan_amount: &f32| {
        return loan_amount * &base_rate
    };
    println!("the total interest to be paid is: {}", calculate_interest(&32567.6));


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

fn print_users(users: &Vec<User>) -> () {
    println!("---- List of Users ----");
    for i in users {
        println!("  {}", i.username);
    }
    println!("--------- END ---------")
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

