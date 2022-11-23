fn main () {
    let linguagem = "Rust";

    let proposito = match linguagem {
        "Rust" => "Rescrever o mundo",
        "PHP" => "Web",
        "Javascript" => "Web",
        "Python" => "Data science",
        "C" => "Sistemas embarcados",
        "C#" => "Windows"
        _ => "Desconhecido"
    };

    println!("O propósito de {} é {}", linguagem, proposito);
}