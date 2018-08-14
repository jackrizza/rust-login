#[macro_use]
extern crate mysql;
extern crate crypto;
extern crate rand;

use mysql as my;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use rand::prelude::*;

struct User {
    username: String,
    password: String,
    hash: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Row {
    id: i32,
    username: String,
    password: String,
    hash: String,
}

struct signed_in_user {
    username : String,
    token : String,
}

fn connection() -> my::Pool {
    let db = my::Pool::new("mysql://rust:rust@127.0.0.1/rust").unwrap();

    // return working database
    db
}

fn random_hash() -> String {
    // registering random generator
    let mut rng = thread_rng();
    // vector of the alphabet
    let _char_vec = vec![
        [
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ],
        [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "v", "w", "X", "y", "Z",
        ],
    ];
    // setting up the new hash
    let mut new_hash = String::new();
    // looping 12 times to make a string 12 long
    for _ in 0..12 {
        // getting a random character from the alphabet
        // and pushing it into the hash string
        new_hash.push_str(&_char_vec[rng.gen_range(1, 2)][rng.gen_range(0, 26)]
            .clone()
            .to_string());
    }

    // returning completed hash
    new_hash
}

fn create_password(_password: String, _hash: String) -> String {
    // putting _password and _hash together
    let mut pass = _password.to_string();
    pass.push_str(&_hash.to_string());
    // setting up hashing algorithm
    let mut hasher = Sha3::sha3_256();
    // making hash
    hasher.input_str(&pass);
    let password = hasher.result_str();

    // returning new password
    password
}

fn check_user_name(username: String) -> Result<String, String> {
    // get all rows in the database with the same username in a database
    let _row_count_vec: Vec<Row> = connection()
        .prep_exec(
            r#"SELECT `id`,`username` FROM `rust`.`login` WHERE username=:user"#,
            params!{"user" => &username.to_string()},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, username) = my::from_row(row);
                    Row {
                        id: id,
                        username: username,
                        password: "Not needed".to_string(), // safety
                        hash: "Not needed".to_string(), // safety
                    }
                })
                .collect()
        })
        .unwrap();
    // the length of the row count array is the row count
    let row_count: i32 = _row_count_vec.len() as i32;
    // if a user exists in the database with the username it will come up as greater than 0
    if row_count > 0 {
        Err("The username was already assigned".to_string())
    } else {
        Ok(username)
    }
}

fn create_account(_new_username: String, _new_password: String) -> Result<User, String> {
    // create a new hash
    let new_hash = random_hash();
    // create a password
    let new_password = create_password(_new_password, new_hash.to_string());
    let new_username = check_user_name(_new_username);
    // if error pass the error
    // if ok create and pass a <User>
    match new_username {
        Err(e) => Err(e),
        Ok(x) => Ok(User {
            username: x,
            password: new_password,
            hash: new_hash,
        }),
    }
}

fn new_account_to_db(user: User) -> Result<String, String> {
    connection()
        .prep_exec(
            r#"INSERT INTO login (username, password, hash) VALUES (:username, :password, :hash) "#,
            params!{
                "username" => &user.username,
                "password" => &user.password,
                "hash" => &user.hash
            },
        )
        .unwrap();
    connection().prep_exec(r#"INSERT INTO `rust`.`token` (username, token, date) VALUES (:username, :token, :date)"#, params!{
            "username" => &user.username,
            "token" => "Null".to_string(),
            "date" => "Null".to_string(),  
        }).unwrap();

    // This is not a good error,
    // This is because until updated,
    // there is not a good way
    // to test if the data was uploaded
    if 1 > 3 {
        Err("Error".to_string())
    } else {
        Ok("Done".to_string())
    }
}

fn sign_in(_username: String, _password: String) -> Result<signed_in_user, String> {
    // first, get users unique hash
    let user: Vec<Row> = connection()
        .prep_exec(
            r#"SELECT * FROM `rust`.`login` WHERE username=:username"#,
            params! {"username" => &_username},
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (id, username, password, hash) = my::from_row(row);
                    Row {
                        id: id,
                        username: username,
                        password: password,
                        hash: hash
                    }
                })
                .collect()
        })
        .unwrap();

    // second, recreate password
    let password: String = create_password(_password, user[0].hash.to_string());
    // third test
    if _username == user[0].username.to_string() && password == user[0].password.to_string() {
        // finally, return
        //Ok => new token
        let token: String = random_hash().to_string();
        // insert token into db
        connection().prep_exec(r#"UPDATE `rust`.`token` SET token=:token, date=:date WHERE username=:username"#, params!{
            "token" => &token.to_string(),
            "date" => "Not ready".to_string(),
            "username" => user[0].username.to_string(),
        }).unwrap();
        // add db command to add token to users token column
        Ok(signed_in_user {
            username : user[0].username.to_string(),
            token : token
        })
    } else {
        Err("Not the right username or password".to_string())
    }
}

fn main() {
    // initialize user
    // random data
    for i in ["Bill", "Sam", "Zach", "Dash", "Bob", "Erik", "Peter", "Adam", "Jack"].iter() {
        let user = create_account(i.to_string(), i.to_string());
        match user {
            Ok(user) => match new_account_to_db(user) {
                Ok(x) => println!("{}", x),
                Err(e) => println!("{}", e),
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    // test if user was created

    // test if user can sign in, should return a random hash if so
    match sign_in(String::from("Jack"), String::from("Jack")) {
        Ok(x) => println!("Username : {}   token : {}", x.username, x.token),
        Err(e) => println!("{}", e),
    }
}
