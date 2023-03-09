struct User {
    name: String,
    email: String,
    is_active: bool,
    login_count: u64,
}

fn main() {
    let user = User {
        name: String::from("Nimai"),
        email: String::from("nimaic.dev@gmail.com"),
        is_active: true,
        login_count: 54,
    };

    let user2 = User {
        name: String::from("Nimai Charan"),
        ..user //copy other data from the user variable
    };
    println!("Name : {}", user.name);
    println!("Name : {}", user2.name);
    println!("Name : {}", user2.email);
}
