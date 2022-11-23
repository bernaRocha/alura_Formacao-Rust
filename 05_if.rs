fn main() {
    let idade: u8 = 32;

    let autorizado = true;

    let maior_idade = idade >= 18;

    let tem_18 = idade == 18;

    if idade >= 18 || idade > 16 && autorizado {
        println!("Pode beber e dirigir!")
    } else if idade > 16 && autorizado {
        println!("Pode beber e dirigir.")
    } else {
        println!("Não pode beber nem dirigir");
    }

    let condicao = if idade >= 18 { "maior" } else { "menor" };

    println!("É {} de idade", condicao);
}