use std::{error, fmt::write, fs::File, io::{Write, BufRead, BufReader, ErrorKind}};

fn main() {
    let path: &str = "lines.text";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
        
    };
    write!(output, "Just some worsd").expect("failed to write file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines(){
        println!("{}", line.unwrap());

    }

    let output2 = File::create("rand.text");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Cant create file:{:?}",error),
            },
            _other_error => panic!("Problem opening file:{:?}",error),
        },
    };
}