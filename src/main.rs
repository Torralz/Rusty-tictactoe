use std::io;
use rand::Rng;

enum Casilla {
    Vacio,
    Cruz,
    Circulo,
}

fn main() {
    println!("Empecemos!");
    println!("
     | | 
    -----
     | | 
    -----
     | | 
    ");

    let mut matriz: [[char; 3]; 3] = [[' '; 3]; 3]; // matriz 3x3 con espacios
    let mut terminado = false;

    while !terminado {
        let mut input_str = String::new();
        println!("Elija una casilla (1-9):");
        io::stdin()
            .read_line(&mut input_str)
            .expect("Error de lectura");

        let input: u32 = match input_str.trim().parse() {
            Ok(num) if num >= 1 && num <= 9 => num,
            _ => {
                println!("Entrada inválida, intente de nuevo.");
                continue;
            }
        };

        relleno_matriz(&mut matriz, input, Casilla::Cruz);
        ia(&mut matriz);
        dibujo_matriz(&matriz);
    }
}

fn relleno_matriz(matriz: &mut [[char; 3]; 3], input: u32, estado: Casilla) {
    let mut cont = 1;

    for fila in 0..3 {
        for columna in 0..3 {
            if cont == input && matriz[fila][columna] == ' ' {
                matriz[fila][columna] = match estado {
                    Casilla::Cruz => 'X',
                    Casilla::Circulo => 'O',
                    Casilla::Vacio => ' ',
                };
                return; // salimos una vez asignado
            }
            cont += 1;
        }
    }
}

fn dibujo_matriz(matriz: &[[char; 3]; 3]) {
    println!(
        "
    {}|{}|{}
    -----
    {}|{}|{}
    -----
    {}|{}|{}
        ",
        matriz[0][0],
        matriz[0][1],
        matriz[0][2],
        matriz[1][0],
        matriz[1][1],
        matriz[1][2],
        matriz[2][0],
        matriz[2][1],
        matriz[2][2]
    );
}

fn ia(matriz: &mut [[char; 3]; 3]){
    let mut encontrado: bool = false;
    for x in 0..3{
        for y in 0..3{
            if matriz[x][y] == 'O'{
                encontrado = true;
                if x > 0 && y > 0{
                    for m in x-1..=x+1{
                        for n in y-1..=y+1{
                            if (0..3).contains(&m) && (0..3).contains(&n){
                                if matriz[m][n] == ' '{
                                    matriz[m][n] = 'O';
                                    return;
                                }
                            }
                        }
                    }
                } 
                else{
                    for m in x..=x+1{
                        for n in y..=y+1{
                            if (0..3).contains(&m) && (0..3).contains(&n){
                                if matriz[m][n] == ' '{
                                    matriz[m][n] = 'O';
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !encontrado{
        let mut rng = rand::thread_rng(); // Corregido: usar thread_rng()
        loop {
            let fila: usize = rng.gen_range(0..3); // Cambiado a usize y rango 0..3
            let colu: usize = rng.gen_range(0..3); // Cambiado a usize y rango 0..3
            if matriz[fila][colu] == ' ' {
                matriz[fila][colu] = 'O'; // La IA juega con 'O'
                break;
            }
        }
    }
}
