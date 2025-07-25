pub struct Student(
    pub i32, pub String, pub String
);

pub fn id(student: &Student) -> u32 {
    student.0.try_into().unwrap()
    // OR
    // student.0 as u32
}

pub fn first_name(student: &Student) -> &str {
    &student.1
}

pub fn last_name(student: &Student) -> &str {
    &student.2
}