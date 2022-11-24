fn ownership() {
    // A variável não possui o valor, possui um ponteiro para um espaço de memória 
    // na heap com o valor, se a string fosse grande demais causaria um stackoverflow

    // string estática let s_string = ("Bernardo");

    // string dinâmica são alocadas na heap
    // Na head o ciclo de vida é maior que na stack
    let s_string = String::from("Bernardo");
    rouba(s_string);
}

fn rouba(string: String) {
    println!("{}", string);
}


fn main() {
    ownership();
}