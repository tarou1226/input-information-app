use std::io::{ Write, stdin, stdout};

fn main() {
    let mut buf = String::new();

    loop {
        print!("input -> ");
        stdout().flush().unwrap();
        let byte = stdin().read_line(&mut buf).unwrap();

        if byte == 0 {
            break;
        }

        print!("{}", buf);
        buf.clear();
    }

}
