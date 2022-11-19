fn main() {
     **************************
     Primera version, prueba!
     **************************
    println!("Por favor intoduce tu nombre: ");

    let mut nombre : String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string();

    // Get age
    println!("Por favor introduce tu edad: ");
    let mut edad : String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    // Convert age to number
    let edad_int : u8 = edad.trim().parse().unwrap();

    if edad_int >= 18 && edad_int != 30 {
        println!("Puedes entrar a la discoteca");

        if edad_int > 74 {
            println!("Deberias estar considerando no ir a la disco");
        }
    } else if edad_int == 30 {
        println!("Vamo!!!!");
    } else {
        println!("Tienes {} anios", edad_int);
    }

    println!("Hola, bienvenido o bienvenida {}, de {} anios", nombre, edad_int);


     **************************
     Loop
     **************************
    // Tow numbers of sum
    let numero_1 = 123;
    let numero_2 = 123;
    let suma = numero_1 + numero_2;

    loop {
        // Show two numbers
        println!("Por favor escribir la suma de {} y {}: ", numero_1, numero_2);

        // Get number to user
        let mut suma_usuario = String::new();
        std::io::stdin().read_line(&mut suma_usuario).unwrap();

        let suma_usuario_int : i16 = suma_usuario.trim().parse().unwrap();

        if suma_usuario_int == suma {
            println!("Lo has hecho muy bien, el resultado {} es correcto", suma);
            break;
        } else {
            println!("El resultado {} no es correcto por favor intentalo de nuemo", suma_usuario_int)
        }
    }
}
