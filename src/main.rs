use std::io::{self, Write};

/*
Cipher Security App

[key][::-1](u16%10)
*/

fn main() {
    let mut cipher = String::new();
    let mut text = String::new();
    let mut key = String::new();

    let mut result = String::new();

    print!("Read(0)/Write(1): ");
    
    io::stdout().flush().unwrap();

    let mut res = String::new();

    io::stdin().read_line(&mut res);

    if res.trim() == "0" {
        print!("Print key: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut key);

        print!("Print sipher: no work ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut cipher);

        let mut c_key = 0;
        for el in key.chars() {
            c_key += (el as u16)%10000;
        }
        let mut sh = String::new();
        sh = cipher.chars().rev().collect::<String>();

        //let mut item;

        for i in 0..sh.len()/4 {
            //item = 1000*(sh[i])
        }
            
    }
    else { 
        print!("Print key: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut key);

        print!("Print text: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut text);
        //cipher = String::from(key);

        let mut c_key = 0;
        for el in key.chars() {
            c_key += (el as u16)%10000;
        }
        print!("{}", c_key);
        for el in text.chars().rev() { 
            print!("{}", ((el as u16) + c_key)%10000);
            c_key = ((el as u16) + c_key)%10000;
        }
        
    }

}
