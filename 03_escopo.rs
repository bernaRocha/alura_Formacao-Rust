const PI:f32 = 3.141592;
static mut GLOBAL:u8 = 1;

fn escopo() {
    println!("PI com 6 casas: {}", PI);

    unsafe {
        println!("Uma variável global: {}", GLOBAL);
    }
}

fn sombra() {
    let a = 123;

    {
        let b = 61;
        println!("dentro das chaves, b = {}", b);
        let a = 999;
        println!("dentro das chaves, a = {}", a);
    }
    //println!("fora das chaves, b = {}", b);
    // error[E0425]: cannot find value `b` in this scope
    println!("fora das chaves, a = {}", a);
}

fn main() {
    escopo();
    sombra();
}
