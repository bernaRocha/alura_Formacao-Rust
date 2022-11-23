fn main() {
    println!("Olá mundo!");
    let variavel:i8 = 127;
    let uns_int:u8 = 128;
    println!("Variavel = {}", variavel);
    println!("Unsined integer = {}", uns_int);
    let semtipo = 90;
    println!("Sem um tipo específico = {}", semtipo);

    let pi = 3.14;
    println!("Variável float = {} e seu tamanho = {} bytes.", pi, std::mem::size_of_val(&pi));

    let inteiro:i32 = 300;
    println!("Número = {} e seu tamanho = {} bytes.", inteiro, std::mem::size_of_val(&inteiro));

    let decimal:f32 = 9.9;
    println!("Decimal = {} e seu tamanho = {} bytes.", decimal, std::mem::size_of_val(&decimal));

    let booleana:bool = false;
    println!("Tamanho da booleana = {} byte", std::mem::size_of_val(&booleana));

    let mut bool_mutavel:bool = true;
    println!("Booleana = {}", bool_mutavel);

    let letra:char = 'R';
    println!("letra = {}, tamanho = {} bytes", letra, std::mem::size_of_val(&letra));
}