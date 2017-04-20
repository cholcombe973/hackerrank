use std::io;

fn median(v: &Vec<u32>) -> Option<f64> {
    let len = v.len();
    match len {
        0 => None,
        1 => Some(f64::from(v[0])),
        //Even
        len if len % 2 == 0 => {
            let v1 = f64::from(v[(len / 2) - 1]);
            let v2 = f64::from(v[len / 2]);
            Some((v1 + v2) / 2.0)
        }
        //Odd
        len => Some(f64::from(v[len / 2])),
    }
}

fn main() {
    let num_lines = {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff);
        buff.trim().parse::<u32>().unwrap()
    };
    let mut list: Vec<u32> = Vec::new();
    for _ in 0..num_lines {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff);
        let num = buff.trim().parse::<u32>().unwrap();

        // Insert our new element while maintaining sorted order
        let pos = list.binary_search(&num).unwrap_or_else(|e| e);
        list.insert(pos, num);

        // print the median
        let median = median(&list);
        match median {
            Some(m) => {
                println!("{:.*}", 1, m);
            }
            None => {
                println!("Err");
            }
        }
    }
    //println!("list: {:?}", list);
}
