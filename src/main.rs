use std::io::{ Write, stdin, stdout };

struct Information {
    name: String,
    age: i32,
    hobby: String
}

impl Information {
    pub fn new() -> Self {
        Information {
            name: "".to_string(),
            age: 0,
            hobby: "".to_string()
        }
    }

    pub fn input_name(&mut self, input: &str) {
        self.name = input.to_string();
    }

    pub fn input_age(&mut self, input: &i32) -> &str {
        self.age = *input;
        match input {
            0..=5 => "babby",
            6..=18 => "young",
            19..=59 => "adult",
            60..=120 => "granpa",
            _ => panic!("error")
        }
    }

    pub fn input_hobby(&mut self, input: &str) {
        self.hobby = input.to_string();
    }

}

fn main() {
    let mut buf = String::new();
    let mut num = 0;
    let mut info = Information::new();
    loop {
        print!("input name -> ");
        stdout().flush().unwrap();
        let byte = stdin().read_line(&mut buf).unwrap();
        if byte == 0 {
            break;
        }
        info.input_name(&buf.trim());
        println!("{}", info.name);
        buf.clear();

        print!("input age -> ");
        stdout().flush().unwrap();
        let byte = stdin().read_line(&mut buf).unwrap();
        if byte == 0 {
            break;
        }
        num = buf.trim().parse().unwrap();
        let age_class = info.input_age(&num);
        println!("{}", age_class);
        buf.clear();

        print!("input hobby -> ");
        stdout().flush().unwrap();
        let byte = stdin().read_line(&mut buf).unwrap();
        if byte == 0 {
            break;
        }
        info.input_hobby(&buf.trim());
        println!("{}", info.hobby);
        buf.clear();
    }
}
