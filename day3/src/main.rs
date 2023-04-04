use core::fmt::Debug;

trait Email {
    fn send(&self) -> ();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    fn say(&self) {
        println!("{}", &self.username)
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.active, self.email, self.username, self.sign_in_count
        )
    }
}

impl Email for User {
    fn send(&self) -> () {
        println!("{:#?}", &self)
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            active: (*self).active,
            username: (*self).username.to_owned(),
            email: (*self).email.to_owned(),
            sign_in_count: (*self).sign_in_count,
        }
    }
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1.clone()
    };

    user1.send();
    user1.say();
    user2.send();
    user2.say();
}
