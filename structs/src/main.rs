fn main() {
    println!("Hello structs!");

    // Read only user :
    let user1 = User {
        active: true,
        username: String::from("lucfun95"),
        email: String::from("l.savio@hotmail.fr"),
        sign_in_count: 1,
    };

    //Mutable user :
    let mut mutable_user  = User {
        active: false,
        username: String::from("i_am_mutable"),
        email: String::from("mut@mut.com"),
        sign_in_count: 2,
    };

    //Create user using function :
    let fn_user = build_user(String::from("fn@fn.com"), String::from("fn_user"));
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
