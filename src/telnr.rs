use rand::Rng;

pub fn gen(how_much : i64) -> Vec<String> {
    let mut rand = rand::thread_rng();
    let mut returnee : Vec<String> = Vec::new();
    for _ in 0..how_much {
        let mut arr : [f32;10] = [0.0; 10];
        rand.fill(&mut arr);




        returnee.push(format!("01{}",arr.into_iter().map(|a| {
            return format!("{}",(a*10.0).floor())
        }).collect::<String>()));
    }

    return returnee

}
