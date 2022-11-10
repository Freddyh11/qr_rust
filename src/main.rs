

//use std::env;
use std::fs;
use fast_qr::{ECL, Version, QRBuilder};


fn main() {
   
    println!("In file {}", "URL.txt");

    let contents = fs::read_to_string("URL.txt")
        .expect("Should have been able to read the file");

        println!("Contents: \n{}", contents); 

        for _line in contents.lines() {
            let qrcode = QRBuilder::new("URL.txt".into())
            .ecl(ECL::H)
            .version(Version::V03)
            .build();
            
        qrcode.unwrap().print();
        }
       


}
