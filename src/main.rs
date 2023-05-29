use colored::Colorize;
use dialoguer::{console::Term, theme::ColorfulTheme, Confirm, Editor, Input, MultiSelect, Select};
enum RegisterStep {
    Info,
    Terms,
    Done,
}

#[derive(Clone)]
struct User {
    username: String,
    email: String,
    password: String,
    terms: bool,
}
impl User {
    fn new(username: String, email: String, password: String, terms: bool) -> Self {
        Self {
            username,
            email,
            password,
            terms,
        }
    }
}

struct Todo {
    title: String,
    completed: bool,
}

impl Todo {
    fn new(title: String, completed: bool) -> Self {
        Self { title, completed }
    }
}

struct UserSpace {
    user: User,
    todos: Vec<Todo>,
}

struct Database {
    users_data: Vec<UserSpace>,
}
impl Database {
    fn new() -> Self {
        Self {
            users_data: Vec::new(),
        }
    }
    fn add_user(&mut self, user: User) -> User {
        let new_user = user.clone();
        self.users_data.push(UserSpace {
            user: new_user,
            todos: Vec::new(),
        });
        user
    }
}

fn interact_selection<'a>(prompt: &str, items: Vec<&'a str>) -> &'a str {
    let selection: Option<usize> = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .with_prompt(prompt)
        .interact_on_opt(&Term::stderr())
        .unwrap();

    match selection {
        Some(index) => items[index],
        None => panic!("User did not select anything"),
    }
}

fn interact_text(prompt: &str) -> String {
    let input = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("dialoguer")
        .interact_text()
        .unwrap();
    input
}
fn main() {
    let users: Vec<User> = Vec::new();
    let todos: Vec<Todo> = Vec::new();

    let mut database = Database::new();
    // login | doesn't have account yet
    let res1 = interact_selection(
        "Join our project today!",
        ["Login", "Doesn't have a account yet"].to_vec(),
    );
    match res1 {
        "Login" => println!("logged in successfully"),
        _ => {
            let new_user = register_user();
            let new_user = database.add_user(new_user);
            dashboard(&mut database, new_user)
        }
    }
}

fn dashboard(database: &mut Database, user: User) {
    // later
    println!("\x1b[93m Welcome to Todo \x1b[0m");
    loop {
        let mut part = 0;
        match part {
            0 => {},
            1 => {},
            2 => {},
            3 => {},
            _ => break,
        }
    }
    // let res = interact_selection("Welcome", items)
}

fn register_user() -> User {
    let username = interact_text("Enter username:");
    let email = interact_text("Enter email:");
    let password = interact_text("Enter password:");
    let confirm_password = interact_text("Confirm password:");

    match password != confirm_password {
        true => panic!("Passwords don't match"),
        false => {}
    }

    let terms = Confirm::new()
        .with_prompt("are you agree to terms of service")
        .interact()
        .unwrap();

    if terms {
        let user = User::new(username, email, password, terms);
        return user;
    } else {
        println!("You must agree to terms of service");
        return register_user();
    }
}
