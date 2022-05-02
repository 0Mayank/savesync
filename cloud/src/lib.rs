#![allow(dead_code)]
#![allow(unused_variables)]

pub struct User {
    todo!()
}

pub fn auth(username: &str, password: &str) -> Result<User, std::Error>{
    todo!()
}

pub fn upload(user: User) -> Result<(), std::Error>{
    todo!()
}

pub fn download(user: User, date: Option<&str>) -> Result<&str, std::Error> {
    todo!()
}

pub fn delete(user: User, date: Option<&str>) -> Result<(), std::Error>{
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
