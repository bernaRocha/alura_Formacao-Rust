fn repeticoes(){
    let multiplicador:u8 = 5;

    let mut contador:u8 = 0;
    while contador < 10 {
        contador += 1;
        println!("{} * {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;
        println!("{} * {} = {}", multiplicador, contador, multiplicador * contador);
        if contador == 10 {
            break;
        }
    }

    let m:u8 = 7;
    for i in 1..=10 {
        //   1..=10  == 1..11
        println!("{} * {} = {}", m, i, m * i);
    }
}

fn main() {
    repeticoes();
}