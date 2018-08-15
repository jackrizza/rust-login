# Authenticator

This is a small project I have been working on for collectivly 38 hours on the weekends and at night

It requires 2 `mysql` databases :

#### CREATE DATABSE
```sql
CREATE DATABASE `rust`;
USE `rust`;
```


#### LOGIN
```sql 
CREATE TABLE IF NOT EXISTS `rust`.`login` (
  `id` INT(11) UNSIGNED NOT NULL AUTO_INCREMENT,
  `username` VARCHAR(64) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_0900_ai_ci' NOT NULL DEFAULT '',
  `password` VARCHAR(64) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_0900_ai_ci' NOT NULL DEFAULT '',
  `hash` VARCHAR(64) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_0900_ai_ci
```

#### TOKEN
```sql 
CREATE TABLE IF NOT EXISTS `mydb`.`token` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `username` VARCHAR(64) NOT NULL,
  `token` VARCHAR(64) NOT NULL,
  `Date` VARCHAR(64) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB
```

#### To point to your mysql server
in the code you will see `fn connection()` 
```rust
fn connection() -> my::Pool {
    let db = my::Pool::new("[PUT YOUR MYSQL URL HERE]").unwrap();

    // return working database
    db
}

```

There is still a huge list of todo's. Please feel free to mess with and commit your idea's

### Todo
| ID         | DESCRIPTION                                        | ACTION |
| :----------| :-------------------------------------------------: |-----:|
| 1          | Create a random hash algorithm                     | ✓ |
| 2          | Create a password hash function                    | ✓ |
| 3          | Create a user                                      | ✓ |
| 4          | autheticate user                                   | ✓ |
| 5          | give user a session-based token                    | ✓ |
| 6          | sign out function                                  | ☐ |
| 7          | add database functionality                         | ✓ |
| 9          | better Results for new_account_to_db()             | ☐ |
| 10         | create authenticator module                        | ☐ |
| 12         | check password strength                            | ☐ |
| 13         | Add a error if connection() does not connect       | ☐ | 
| 14         | Move all database calls to seperate functions      | ☐ |