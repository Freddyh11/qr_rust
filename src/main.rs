

//use std::env;
use std::fs;
use fast_qr::QRBuilder;


fn main() {
   
    println!("In file {}", "URL.txt");

    let contents = fs::read_to_string("URL.txt")
        .expect("Should have been able to read the file");

        println!("Contents: \n{}", contents); 

        for line in contents.lines() {
            let qrcode = QRBuilder::new(line.to_string()).build();
            
        qrcode.unwrap().print();
        }
       


}
