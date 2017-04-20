use std::io;

fn main() {
    let num_lines = {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff);
        buff.trim().parse::<u32>().unwrap()
    };
    'outer: for _ in 0..num_lines {
        let mut stack: Vec<char> = Vec::new();
        let mut buff = String::new();
        io::stdin().read_line(&mut buff);
        'inner: for c in buff.chars() {
            if c == '{' || c == '(' || c == '[' {
                stack.push(c);
            }
            if c == '}' {
                match stack.pop() {
                    Some(l) => {
                        if l != '{' {
                            println!("NO");
                            continue 'outer;
                        }
                    }
                    None => {
                        println!("NO");
                        continue 'outer;
                    }
                }
            } else if c == ')' {
                match stack.pop() {
                    Some(l) => {
                        if l != '(' {
                            println!("NO");
                            continue 'outer;
                        }
                    }
                    None => {
                        println!("NO");
                        continue 'outer;
                    }
                }
            } else if c == ']' {
                match stack.pop() {
                    Some(l) => {
                        if l != '[' {
                            println!("NO");
                            continue 'outer;
                        }
                    }
                    None => {
                        println!("NO");
                        continue 'outer;
                    }
                }
            }
        }
        if stack.len() == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
