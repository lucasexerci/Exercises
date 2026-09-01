pub fn is_armstrong_number(num: u32) -> bool {
    let mut vetor = Vec::new();

    for c in num.to_string().chars() {
        vetor.push(c.to_digit(10).unwrap());
    }

    let casas = vetor.len() as u32;

    let mut soma: u32 = 0;
    for &digito in &vetor {
        soma += digito.pow(casas);
    }

    soma == num
}