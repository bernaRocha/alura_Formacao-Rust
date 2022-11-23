const PI:f32 = 3.141592;
static mut GLOBAL:u8 = 1;

fn main() {
    println!("PI com 6 casas: {}", PI);

    unsafe {
        println!("Uma variável global: {}", GLOBAL);
        // Unsafe devido o mut
    }

}