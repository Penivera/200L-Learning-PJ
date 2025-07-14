#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub trait Surname<T> {
    fn surname(&mut self, other: T);
}

impl<T> Surname<T> for User
where
    T: Into<String>,
{
    fn surname(&mut self, other: T) {
        self.name = format!("{} {}", self.name, other.into());
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub reg_no: String,
    pub level: String,
    pub department: String,
    pub faculty: String,
}
