// 构建这模式, 是把对象的各个部分, 通过一步一步调用 Builder 的方法构建一个完整的对象

#![allow(unused)]
#[derive(Debug, PartialEq)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    // This method will help users to discover the builder
    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}

#[derive(Default)]
pub struct UserBuilder {
    id: u32,
    name: String,
    email: String,
}

impl UserBuilder {
    pub fn new() -> UserBuilder {
        UserBuilder {
            id: 1,
            name: String::from("username"),
            email: String::from("username@domain.com"),
        }
    }

    pub fn id(mut self, id: u32) -> UserBuilder {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> UserBuilder {
        self.name = name;
        self
    }

    pub fn email(mut self, email: String) -> UserBuilder {
        self.email = email;
        self
    }

    pub fn build(self) -> User {
        User {
            id: self.id,
            name: self.name,
            email: self.email,
        }
    }
}
fn builder_test() {
    let user = User {
        id: 1,
        name: "username".to_string(),
        email: "username@domain.com".to_string(),
    };
    let user_from_builder: User = UserBuilder::new()
        .id(1)
        .name(String::from("username"))
        .email(String::from("username@domain.com"))
        .build();
    // assert_eq!(user, user_from_builder);
    println!("new user by literal: {:?}", user);
    println!("new user by builder: {:?}", user_from_builder);
}
fn main() {
    builder_test()
}
