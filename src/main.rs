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
        let ganadorCasilla: i32;
        relleno_matriz(&mut matriz, input, Casilla::Cruz);
        let (fila, columna) = ia(&mut matriz);
        matriz[fila][columna] = 'O';
        (terminado, ganadorCasilla) = comprobar_ganador(&matriz);
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

fn ia(matriz: &mut [[char; 3]; 3])->(usize, usize){
    let mut mejorPuntuacion = i32::MIN;
    let mut mejorJugada = (0,0);
    for (fila, columna) in posibles_movimientos(matriz){
        matriz[fila][columna] = 'O';
        let puntuacion = minMax(matriz, 'X');
        matriz[fila][columna] = ' ';
        if puntuacion > mejorPuntuacion{
            mejorPuntuacion = puntuacion;
            mejorJugada = (fila, columna);
        }
    }
    mejorJugada
}

fn minMax(matriz: &mut [[char; 3]; 3], jugador: char) -> i32{
    let (terminado, ganador) = comprobar_ganador(&matriz);
    if terminado {
        return ganador;
    }
    let mut mejorJugada;
    
    if jugador == 'O'{//Maximizamos
        mejorJugada = i32::MIN;
        for (fila, columna) in posibles_movimientos(matriz){
            matriz[fila][columna] = 'O';
            let jugada = minMax(matriz, 'X');
            matriz[fila][columna] = ' ';
            mejorJugada = mejorJugada.max(jugada);
        }
    }
    else{
        mejorJugada = i32::MAX;
        for (fila, columna) in posibles_movimientos(matriz){
            matriz[fila][columna] = 'X';
            let jugada = minMax(matriz, 'O');
            matriz[fila][columna] = ' ';
            mejorJugada = mejorJugada.min(jugada);
        }
    }
   mejorJugada 
}

fn posibles_movimientos(matriz: &[[char; 3]; 3] )->Vec<(usize, usize)>{
    let mut movimientos = Vec::new();
    for fila in 0..3{
        for columna in 0..3{
            if matriz[fila][columna] == ' '{
                movimientos.push((fila, columna));
            }
        }
    }
    movimientos
}

fn comprobar_ganador (matriz: & [[char; 3];3]) -> (bool, i32){
    let solFicha: i32;
    //primero comprobamos las horizontales
    if matriz[0][0] == matriz[0][1] && matriz[0][2] == matriz[0][1] && matriz[0][1] != ' ' {
        match matriz[0][0] {
            'X' => solFicha = -10,
            'O' => solFicha = 10,
            _ => todo!(),
        }
        return (true, solFicha); 
    }
    if matriz[1][0] == matriz[1][1] && matriz[1][2] == matriz[1][1] && matriz[1][1] != ' ' {
        match matriz[1][0] {
            'X' => solFicha = -10,
            'O' => solFicha = 10,
            _ => todo!(),
        }
        return (true, solFicha); 
    }
    if matriz[2][0] == matriz[2][1] && matriz[2][2] == matriz[2][1] && matriz[2][1] != ' ' {
        match matriz[2][0] {
            'X' => solFicha = -10,
            'O' => solFicha = 10,
            _ => todo!(),
        }
        return (true, solFicha); 
    }

    //Seguimos comprobando las Verticales
    if matriz[0][0] == matriz[1][0] && matriz[2][0] == matriz[1][0] && matriz[1][0] != ' ' {
        match matriz[0][0] {
            'X' => solFicha = -10,
            'O' => solFicha = 10,
            _ => todo!(),
        }
        return (true, solFicha); 
    }
    if matriz[0][1] == matriz[1][1] && matriz[2][1] == matriz[1][1] && matriz[1][1] != ' ' {
        match matriz[0][1] {
            'X' => solFicha = -10,
            'O' => solFicha = 10,
            _ => todo!(),
        }
        return (true, solFicha); 
    }
    if matriz[0][2] == matriz[1][2] && matriz[2][2] == matriz[1][2] && matriz[1][2] != ' ' {
        match matriz[0][2] {
            'X' => solFicha = -10,
            'O' => solFicha = 10,
            _ => todo!(),
        }
        return (true, solFicha); 
    }

    //Para terminar comprobamos las diagonales
    if matriz[0][0] == matriz[1][1] && matriz[2][2] == matriz[1][1] && matriz[1][1] != ' ' {
        match matriz[0][0] {
            'X' => solFicha = -10,
            'O' => solFicha = 10,
            _ => todo!(),
        }
        return (true, solFicha); 
    }
    if matriz[0][2] == matriz[1][1] && matriz[2][0] == matriz[1][1] && matriz[1][1] != ' ' {
        match matriz[0][2] {
            'X' => solFicha = -10,
            'O' => solFicha = 10,
            _ => todo!(),
        }
        return (true, solFicha); 
    }

    //contemplar el caso de que el tablero esté lleno sin solución
    let mut completo : bool = true;
    for fila in 0..3{
        for columna in 0..3{
            if matriz[fila][columna] == ' '{
                completo = false;
            }
        }

    }
    if completo {
        return (true, 0);
    }

    (false, 0)
}
