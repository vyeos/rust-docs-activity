#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
enum Role {
    Student,
    Educator,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
enum Experience {
    Beginner,
    Intermediate,
    Professional,
}

impl fmt::Display for Experience {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

struct Developer {
    id: u32,
    name: String,
    role: Role,
    rust_xp: Experience,
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "===== DEV CARD =====\n{a:<9}: {name}\n{b:<9}: {role}\n{c:<9}: {xp}\n{d:<9}: {id:06b}\n====================",
            a = "Name",
            name = self.name,
            b = "Role",
            role = self.role,
            c = "Rust XP",
            xp = self.rust_xp,
            d = "Binary ID",
            id = self.id
        )
    }
}

fn main() {
    let dev = Developer {
        id: 14,
        name: String::from("Alice"),
        role: Role::Student,
        rust_xp: Experience::Beginner,
    };

    println!("{}", dev);
}
