
struct User {
    active : bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main (){
    let mut User1 : User = build_user(
         String::from("rajdeep"),
         String::from("rajdeep.singh@gmail.com"),
    );
    
    println!("the username is {}", User1.username);

    let User2 = User {
       email: String::from("another@gmail.com"),
        ..User1
    };
    println!("the username is {}", User2.username);
    println!("the email is {}", User2.email);
  
}


fn build_user(username:String, email: String ) -> User{
    User {
        active: true,
        username: username,// username,
        email: email,
        sign_in_count: 0,
    }
}