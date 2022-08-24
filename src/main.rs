//libreria para operaciones de io, se importa por no ser parte del preludio
use std::io;

fn main() {
    println!("=== Convertidor de Fahrenheit a Celsius ===");
    

    
    let mut execution_flag: bool = true;

    while execution_flag {
        let mut fahrenheit_grades = String::new();

        println!("Introudce la cantidad de grados fahrenheit: ");
        io::stdin()
        .read_line(&mut fahrenheit_grades)
        .expect("Error al obtener valor de entrada de terminal.");

        let fahrenheit_grades: f64 = match fahrenheit_grades.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Introduce un numero valido");
                continue; // continue se puede usar con while (igual que loop)
            },
        };
        //en este caso nos es de utilidad definir los componentes de a division como punto flotante
        //en caso contrario se tomara como entero y se redondeara
        //ademas de que la multiplicacion entre f64 e integer no esta implementada
        let celcius_grades = (fahrenheit_grades - 32.0) * (5.0 / 9.0);
        let emoji = get_emoji_weather(celcius_grades);
        println!("{fahrenheit_grades}°F es equivalente a {celcius_grades}°C {emoji}");
        execution_flag = false;
    }
    
}

fn get_emoji_weather(celsius_grades: f64) -> char {
    if celsius_grades > 28.0 {
        '🥵'
    } else if celsius_grades < 5.0 {
        '🥶'
    } else {
        '😎'
    }
}
