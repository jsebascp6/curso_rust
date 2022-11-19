fn sumar_uno(numero: i32) -> i32 {
    let numero_final = numero + 1;
    println!("{}", numero_final);

    return numero_final;
}

fn main() {
    println!("Hola");

    sumar_uno(10);
}
