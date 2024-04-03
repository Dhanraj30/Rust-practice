fn main() {
    let mut company = "Tutorials".to_string();
    company.push_str(" Point end \r\n");
    println!("{}",company);
    println!("{}",company.len());
    println!("after trim :{}",company.trim().len());

    let mut i = 1;
    for token in company.split_whitespace(){
        println!("token {} {}",i,token);
        i+=1;
    }
    //store in vec

    let tokens:Vec<&str> = company.split_whitespace().collect();
    println!("first is {}",tokens[0]);
    println!("second is {}",tokens[1]);

    //let example = vec![e, x, a, m, p, l, e];
    //print chars

    for n in company.chars(){
        println!("{}", n);
    }
}