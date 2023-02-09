use rand::Rng;



pub fn gen_email(how_much : usize) -> Option<Vec<String>> {
    let mut emails : Vec<String> = Vec::with_capacity(how_much);
    let alphabet : Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let domain = "gmail.com";
    let mut rand_gen = rand::thread_rng();

    for _ in 0..how_much {
        let mut email : String = "".to_string();
        let mut arr : [f32;10] = [0.0; 10];
        rand_gen.fill(&mut arr);

        for n in arr {
            email = format!("{}{}",email,alphabet.get((n * 26.0) as usize)?);
        }
        emails.push(format!("{}@{}",email,domain));
    }
    return Some(emails)
}
