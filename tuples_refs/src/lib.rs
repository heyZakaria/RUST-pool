pub struct Student{
    id:u32,
    first_name:&str,
    last_name:&str,
}
pub fn id(student: &Student) -> u32 {
    student.id
}

pub fn first_name(student: &Student) -> &str {
    student.first_name
}

pub fn last_name(student: &Student) -> &str {
    student.last_name
}