extern crate regex;

use self::regex::Regex;
use std::num::ParseIntError;


mod defs;



/*
variables globales que se usaran para controlar el tablero
La estructura reflejará la situación del tablero despues de cada jugada
*/

#[derive(Copy, Clone)]
pub struct Tablero {
    board_array: [i16; 128],             // el tablero 0x88
    to_move: i16,                        // el turno, 1 = blanco, -1 = negro
    en_passant: i16,                     // captura al paso
    white_castle: i16,                  // enroque blanco ¿ambos, solo corto/largo, ninguno?
    black_castle: i16,                  // enroque negro - idem
    moves_fifty: i16,                    // regla 50 jugadas (penultimo campo FEN)
    moves_full: i16                      // num. jugadas de la partida (ultimo campo FEN)
}

impl Tablero {
    pub fn init() -> Tablero{
        Tablero{
            board_array: [0; 128],
            to_move: 1,
            en_passant: -1,
            white_castle: defs::CASTLE_NONE,
            black_castle: defs::CASTLE_NONE,
            moves_fifty: 0,
            moves_full: 0
        }
    }
    
    fn modif_board(&mut self, indice: usize, pieza: i16){
        self.board_array[indice] = pieza;
    }
    
    fn modif_bando(&mut self, bando: i16){
        self.to_move = bando;
    }
    
    fn modif_enroque_blanco(&mut self, estatus: i16){
        self.white_castle = estatus;
    }
    
    fn modif_enroque_negro(&mut self, estatus: i16){
        self.black_castle = estatus;
    }
    
    fn modif_paso(&mut self, estatus: i16){
        self.en_passant = estatus;
    }
    
    fn modif_medias(&mut self, num: i16){
        self.moves_fifty = num;
    }
    
    fn modif_todas(&mut self, num: i16){
        self.moves_full = num;
    }
}


// ========= la estructura del movimiento ==================
#[derive(Copy, Clone)]
pub struct Movim {
    pub piece_moving : i16,          // La pieza que se está moviendo
    pub from_index : usize,          // La casilla desde la que se mueve la pieza
    pub to_index : usize,            // La casilla a la que se mueve la pieza
    pub capture : i16,              // Valor de la pieza capturada (0 si no captura)
    pub move_type : i16,             // Tipo de movim.: ordinario, enroque, al-paso ... (ver en defs.rs)
    pub prev_pos : [i16; 4]          // Mantiene la posición antes del movimiento 
}                                   // (utilizada para deshacer jugada)
                                    // Un array con 4 elementos con información de la posición previa:
                                    // prev_pos[0] = al paso --> -1 = no hay captura al paso
                                    // prev_pos[1] = derechos de enroque blancos
                                    // prev_pos[2] = derechos de enroque negros
                                    // prev_pos[3] = medias-jugadas

impl Movim {
    pub fn init(pieza: i16, desde: usize, hasta: usize, captura: i16, 
            tipo_movim: i16, pos_previa: [i16; 4]) -> Movim {
        Movim{
            piece_moving: pieza,
            from_index : desde,
            to_index : hasta,
            capture : captura,
            move_type : tipo_movim,
            prev_pos : pos_previa
        }
    }
}

// ===========================================================


/******************************************/
/** Metodos de manipulación de la FEN    **/
/******************************************/

pub fn setup_inicio(mut board: &mut Tablero){
    let _resultado = set_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &mut board);
}


pub fn set_fen(fen: &str, board: &mut Tablero) -> bool{
    let fen_valida: bool;   // = true;
    fen_valida = valida_fen(fen);
    if !fen_valida{
        panic!("La cadena FEN no es valida {}", fen_valida);
    }
    
    // dividimos la fen en seis trozos
    let mut var_fen = String::from(fen);
    
    var_fen = var_fen.trim().to_string();
    let mut iter = var_fen.split_whitespace();
    /*
    iter.next(), iter.next(), iter.next(), iter.next());
    let fen_dividida = iter.collect::<Vec<&str>>();   
    */
    let mut partes = iter.next();
    let parte1: &str;   // = "";
    
    // TODO quitar estas comprobaciones ya que se valida la FEN con una funcion nueva
    // estas comprobaciones parece que son redundantes fn valida_fen
    match partes {
        // si lo ha encontrado
        Some(x) => parte1 = x,
        // no lo ha encontrado
        None    => parte1 = "None",
    }
    
    partes = iter.next();
    let parte2: &str;   // = "";
    match partes {
        // si lo ha encontrado
        Some(x) => parte2 = x,
        // no lo ha encontrado
        None    => parte2 = "None",
    }
    
    partes = iter.next();
    let parte3: &str;   // = "";
    match partes {
        // si lo ha encontrado
        Some(x) => parte3 = x,
        // no lo ha encontrado
        None    => parte3 = "None",
    }
    
    partes = iter.next();
    let parte4: &str;   // = "";
    match partes {
        // si lo ha encontrado
        Some(x) => parte4 = x,
        // no lo ha encontrado
        None    => parte4 = "None",
    }
    
    partes = iter.next();
    let parte5: &str;    // = "";
    match partes {
        // si lo ha encontrado
        Some(x) => parte5 = x,
        // no lo ha encontrado
        None    => parte5 = "None",
    }
    
    partes = iter.next();
    let parte6: &str;   // = "";
    match partes {
        // si lo ha encontrado
        Some(x) => parte6 = x,
        // no lo ha encontrado
        None    => parte6 = "None",
    }
    
    
    if parte1 == "None" || parte2 == "None" ||
            parte3 == "None" || parte4 == "None" ||
            parte5 == "None" || parte6 == "None" {
        return false;
    }
    
    let mut i: usize = 0;
    
    /*
    # Las partes de la FEN a tratar son:
        # parte1 - Piezas
        # parte2 - Bando a jugar
        # parte3 - Derechos de enroque
        # parte4 - Casilla al paso
        # parte5 - Medias jugadas (para la regla de las 50 jugadas) y las jugadas completas
    */
    // unas variables para maniobrar
    let mut board_index: usize = 112;
    let mut lista_car: Vec<char> = parte1.chars().collect();
    let mut caracter: char;
    
    while i < lista_car.len(){
        caracter = lista_car[i];
        
        // vemos de que pieza se trata
        if caracter == '/' {
            board_index -= 24usize;
        }
        else if caracter == 'K'{
            board.modif_board(board_index, defs::W_KING);
            board_index += 1;
        }
        else if caracter == 'Q'{
            board.modif_board(board_index, defs::W_QUEEN);
            board_index += 1;
        }
        else if caracter == 'R'{
            board.modif_board(board_index, defs::W_ROOK);
            board_index += 1;
        }
        else if caracter == 'B'{
            board.modif_board(board_index, defs::W_BISHOP);
            board_index += 1;
        }
        else if caracter == 'N'{
            board.modif_board(board_index, defs::W_KNIGHT);
            board_index += 1;
        }
        else if caracter == 'P'{
            board.modif_board(board_index, defs::W_PAWN);
            board_index += 1;
        }
        else if caracter == 'k'{
            board.modif_board(board_index, defs::B_KING);
            board_index += 1;
        }
        else if caracter == 'q'{
            board.modif_board(board_index, defs::B_QUEEN);
            board_index += 1;
        }
        else if caracter == 'r'{
            board.modif_board(board_index, defs::B_ROOK);
            board_index += 1;
        }
        else if caracter == 'b'{
            board.modif_board(board_index, defs::B_BISHOP);
            board_index += 1;
        }
        else if caracter == 'n'{
            board.modif_board(board_index, defs::B_KNIGHT);
            board_index += 1;
        }
        else if caracter == 'p'{
            board.modif_board(board_index, defs::B_PAWN);
            board_index += 1;
        }
        else{
            // convertimos los caracteres numeros a enteros son min=1 y max=8
            // los ascii de los numeros son
            /*
            dec hex bin        car  descrip.
            48 	30 	00110000 	0 	zero
            49 	31 	00110001 	1 	one
            50 	32 	00110010 	2 	two
            51 	33 	00110011 	3 	three
            52 	34 	00110100 	4 	four
            53 	35 	00110101 	5 	five
            54 	36 	00110110 	6 	six
            55 	37 	00110111 	7 	seven
            56 	38 	00111000 	8 	eight
            57 	39 	00111001 	9 	nine
            */
            // por lo tanto al ascii dec hay que restarle 48
            board_index += caracter as usize - 48;
        }
        
        i += 1;
    }
    
    // lo siguiente es el bando que le toca mover
    if parte2 == "w"{
        board.modif_bando(defs::WHITE_TO_MOVE);
    }
    else{
        board.modif_bando(defs::BLACK_TO_MOVE);
    }
    
    // lo siguiente es lo de los enroques
    i = 0;
    lista_car = parte3.chars().collect();
    while i < lista_car.len(){
        caracter = lista_car[i];
        
        // empezamos por el blanco
        if caracter == 'K'{     // blanco al menos puede enrocarse corto
            board.modif_enroque_blanco(defs::CASTLE_SHORT);
        }
        else if caracter == 'Q'{    // el blanco al menos puede enrocarse largo
            // Si el blanco ya puede enrocarse corto, hacemos ambos, si no solo largo
            if board.white_castle == defs::CASTLE_SHORT{
                board.modif_enroque_blanco(defs::CASTLE_BOTH);
            }
            else{
                board.modif_enroque_blanco(defs::CASTLE_LONG);
            }
        }
        // seguimos con los enroques del negro. Es la misma mecanica
        if caracter == 'k'{
            board.modif_enroque_negro(defs::CASTLE_SHORT);
        }
        else if caracter == 'q'{
            if board.black_castle == defs::CASTLE_SHORT{
                board.modif_enroque_negro(defs::CASTLE_BOTH);
            }
            else{
                board.modif_enroque_negro(defs::CASTLE_LONG);
            }
        }
        
        i += 1;
    }
    
    // ahora toca la casilla al paso
    i = 0;
    let mut desplaza088: i16;   // = 0;
    lista_car = parte4.chars().collect();
    if lista_car.len() < 2 && lista_car[0] == '-'{
        board.modif_paso(-1);
    }
    else if lista_car.len() == 2 {
        while i < lista_car.len(){
            caracter = lista_car[i];
            
            if caracter == 'a'{
                board.modif_paso(0);
            }
            else if caracter == 'b'{
                board.modif_paso(1);
            }
            else if caracter == 'c'{
                board.modif_paso(2);
            }
            else if caracter == 'd'{
                board.modif_paso(3);
            }
            else if caracter == 'e'{
                board.modif_paso(4);
            }
            else if caracter == 'f'{
                board.modif_paso(5);
            }
            else if caracter == 'g'{
                board.modif_paso(6);
            }
            else if caracter == 'h'{
                board.modif_paso(7);
            }
            // ahora empezamos con las filas de la casilla al paso
            // en la fila 3 o bien en la 6
            else if caracter == '3'{
                // añade 2 filas al index
                board.en_passant += 32;
                desplaza088 = board.en_passant;
                
                board.modif_paso(desplaza088);
            }
            else if caracter == '6'{
                // Añade 5 filas al indice: 5x16
                board.en_passant += 80;
                desplaza088 = board.en_passant;
                
                board.modif_paso(desplaza088);
            }
            
            i += 1;
        }
    }
    
    // ahora seguimos con las medias jugadas y el total de ellas
    //parte5
    let mut jug: i16 = parte5.parse().unwrap();
    board.modif_medias(jug);
    
    // ahora por ultimo el total de movims
    //parte6
    jug = parte6.parse().unwrap();
    board.modif_todas(jug);
    
    true                                   
}


pub fn get_fen(board: &mut Tablero) -> String{
    // Para almacenar la cadena FEN
    let mut fen_string = "".to_string();
    
    //Las siguientes lineas añade las piezas y casillas en blanco a la FEN
        
    let mut index: i16 = 112;     //# Realiza un seguimiento del índice en el tablero.
    let mut empties: u8 = 0;        //# Numero de casillas vacías en una fila
    let mut caracter: char;         // para convertira a caracter los nums de la fen
    
    while index >= 0{               //# Recorre hasta el final del tablero real
        if index & 0x88 != 0{       //# Hemos llegado al final de una fila
            if empties != 0{
                // con ascii hay una diferencia de 48
                caracter = (48 + empties) as char;
                fen_string.push(caracter);       //#Añade el numero de empties number si no es 0
                empties = 0;
            }
            //if index < 24{break;}
            index -= 24;                        //# Salta a la fila siguiente
            if index >= 0{
                fen_string.push('/');           //# Añade la marca para una fila nueva, si no es el final
            }
        }
        else{
            if board.board_array[index as usize] != defs::EMPTY_SQUARE{    //# Si hay una pieza en la casilla
                if empties != 0{
                    caracter = (48 + empties) as char;
                    fen_string.push(caracter);                  //# Añade el num. de casillas vacías
                }
                empties = 0;                //# Reseteamos la empties (ya que no está llegando una pieza)
            }
            // Añadimos la inicial de la pieza
            if board.board_array[index as usize] == defs::W_KING{
                    fen_string.push('K');
            }
            else if board.board_array[index as usize] == defs::W_QUEEN{
                fen_string.push('Q');
            }
            else if board.board_array[index as usize] == defs::W_ROOK{
                fen_string.push('R');
            }
            else if board.board_array[index as usize] == defs::W_BISHOP{
                fen_string.push('B');
            }
            else if board.board_array[index as usize] == defs::W_KNIGHT{
                fen_string.push('N');
            }
            else if board.board_array[index as usize] == defs::W_PAWN{
                fen_string.push('P');
            }
            else if board.board_array[index as usize] == defs::B_KING{
                fen_string.push('k');
            }
            else if board.board_array[index as usize] == defs::B_QUEEN{
                fen_string.push('q');
            }
            else if board.board_array[index as usize] == defs::B_ROOK{
                fen_string.push('r');
            }
            else if board.board_array[index as usize] == defs::B_BISHOP{
                fen_string.push('b');
            }
            else if board.board_array[index as usize] == defs::B_KNIGHT{
                fen_string.push('n');
            }
            else if board.board_array[index as usize] == defs::B_PAWN{
                fen_string.push('p');
            }
            else{
                empties += 1;
            }
            
            index += 1;
        }
    }
    // FIN de la primera parte de la FEN (piezas)
    fen_string.push(' ');       //# Añadimos espacio para la siguiente parte
    
    //Añadimos el bando a jugar (importante el espacio anterior)
    if board.to_move == defs::WHITE_TO_MOVE{
        fen_string.push('w');       // toca al al blanco
    }
    else{
        fen_string.push('b');       // toca mover al negro
    }
    
    // Añadimos espacio para la siguiente parte
    fen_string.push(' ');
    
    // derechos de enroque
    if board.white_castle == defs::CASTLE_NONE && board.black_castle == defs::CASTLE_NONE{
        fen_string.push('-');       //# Ninguno puede enrocarse
    }
    else{                           //# Al menos un bando puede enrocarse en alguno de los flancos
        if board.white_castle == defs::CASTLE_SHORT{
            fen_string.push('K');
        }
        else if board.white_castle == defs::CASTLE_LONG{
            fen_string.push('Q');
        }
        else if board.white_castle == defs::CASTLE_BOTH{
            fen_string.push('K');
            fen_string.push('Q');
        }
        
        if board.black_castle == defs::CASTLE_SHORT{
            fen_string.push('k');
        }
        else if board.black_castle == defs::CASTLE_LONG{
            fen_string.push('q');
        }
        else if board.black_castle == defs::CASTLE_BOTH{
            fen_string.push('k');
            fen_string.push('q');
        }
    }
    
    //  Añadimos espacio para la siguiente parte
    fen_string.push(' ');
    
    // casilla al paso
    if board.en_passant == -1{
        fen_string.push('-');       //# Si no hay casilla al paso disponible
    }
    else{                           //# Hay una casilla al paso disponible
        if board.en_passant%16 == 0{
            fen_string.push('a');
        }
        else if board.en_passant%16 == 1{
            fen_string.push('b');
        }
        else if board.en_passant%16 == 2{
            fen_string.push('c');
        }
        else if board.en_passant%16 == 3{
            fen_string.push('d');
        }
        else if board.en_passant%16 == 4{
            fen_string.push('e');
        }
        else if board.en_passant%16 == 5{
            fen_string.push('f');
        }
        else if board.en_passant%16 == 6{
            fen_string.push('g');
        }
        else if board.en_passant%16 == 7{
            fen_string.push('h');
        }
        
        if (board.en_passant-(board.en_passant%16))/16 == 2{
            fen_string.push('3');
        }
        else if (board.en_passant-(board.en_passant%16))/16 == 5{
            fen_string.push('6');
        }
    }
    
    // Añadimos espacio para la siguiente parte
    fen_string.push(' ');
    
    // Añadimos los medio movims. desde la ult. captura/movim. de peón
    let mut lista_car: Vec<char> = board.moves_fifty.to_string().chars().collect();
    for x in lista_car {
        fen_string.push(x);
    }
    //fen_string.push(board.moves_fifty.to_string());
    
    fen_string.push(' ');
    
    // Añade numero de jugadas completas efectuadas hasta el momento
    lista_car = board.moves_full.to_string().chars().collect();
    for x in lista_car {
        fen_string.push(x);
    }
    //fen_string.push(board.moves_full.to_string());
    
    
    fen_string
}

fn valida_fen(fen: &str) -> bool{
    // dividimos la fen en seis trozos
    let var_fen = String::from(fen);
    
    let v_fen = var_fen.trim().to_string();
    let iter = v_fen.split_whitespace();
    let fen_dividida = iter.collect::<Vec<&str>>();
    
    //let mut numero: i16;
    if fen_dividida.len() != 6{
        return false;
    }
    
    // ¿ campo numero de jugadas completas es un valor entero > 0? 
    match texto_a_int(fen_dividida[5]){
        Ok(n) => if n < 0{ 
                    return false;
                 },
        Err(_err) => return false,
    }
    
    // las medias jugadas
    match texto_a_int(fen_dividida[4]){
        Ok(n) => if n < 0{ 
                    return false;
                 },
        Err(_err) => return false,
    }
    
    // 4to es una string válida? posibles son '-' o e3, e6, etc es la casilla al paso
    let mut patron = Regex::new(r"^(-|[a-h]{1}[3|6]{1})$").unwrap();
    if !patron.is_match(fen_dividida[3]){
        return false;
    }
    
    // los enroques
    patron = Regex::new(r"(^-$)|(^[K|Q|k|q]{1,}$)").unwrap();
    if !patron.is_match(fen_dividida[2]){
        return false;
    }
    
    // el turno
    patron = Regex::new(r"^(w|b)$").unwrap();
    if !patron.is_match(fen_dividida[1]){
        return false;
    }
    
    // la ultima parte de las pueas
    // primero el iterador
    let mut split = fen_dividida[0].split("/");
    // ahora construyo el array
    let filas = split.collect::<Vec<&str>>();
    if filas.len() < 8 {
        return false;
    }
    let mut sum_fields:i16;             // = 0;
    let mut previous_was_number: bool;  // = false;
    
    for i in 0..filas.len(){
        sum_fields = 0;
        previous_was_number = false;
        
        let s = filas[i];
        let char_vec: Vec<char> = s.chars().collect();
        for k in 0..char_vec.len(){
            if char_vec[k].is_digit(10){        // en base 10
                // cada fila es valida
                if previous_was_number{
                    return false
                }
                //sum_fields += int(char_vec[k], 10);
                let cad = char_vec[k].to_string();
                match texto_a_int(&cad){
                    Ok(n) => sum_fields += n,
                    Err(_err) => return false,
                }
                previous_was_number = true;
            }
            else{
                // comprobar los simbolos de las piezas
                patron = Regex::new(r"^[prnbqkPRNBQK]$").unwrap();
                if !patron.is_match(&char_vec[k].to_string()){
                    return false;
                }
                sum_fields += 1;
                previous_was_number = false;
            }
        }
        // compueba la suma de piezas + casillas en blanco debe ser 8    
        if sum_fields != 8{
            return false;
        }
    }
    
    // por ultomo la casilla al paso:
    // si es el turno blanco la de casilla al paso ser 6 y 3 en caso de que sea el negro
    
    if fen_dividida[3].len() > 1{
        split = fen_dividida[3].trim().split("");   
        //añade dos elem.vacios al principio y final. por tanto...
        let paso = split.collect::<Vec<&str>>();
        if (paso[2] == "3" && fen_dividida[1] == "w") ||
                (paso[2] == "6" && fen_dividida[1] == "b"){
            return false;
        }
    }
    
    
    true
    
    
}

fn texto_a_int(number_str: &str) -> Result<i16, ParseIntError> {
    match number_str.parse::<i16>() {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}


// ============ la parte de make_move y unmake_movim ===========

// antes de generar los posibles movims. legales vamos a construir
// las funcs. make_movim y unmake_movim

fn make_movim(board: &mut Tablero, movim: Movim) -> Movim {
    // anulamos cualquier al paso, luego se positivará si procede
    board.modif_paso(-1); 
    // cambiamos el lado a mover (positivo a negativo y viceversa)
    let bando = board.to_move;
    board.modif_bando(bando *-1);
    
    let mut incr: i16;      //esta variable la utilizaré de comodin
    
    if movim.piece_moving != defs::W_PAWN && movim.piece_moving != defs::B_PAWN && movim.capture == 0{
        incr = board.moves_fifty;
        incr += 1;
        board.modif_medias(incr);       // Incrementa si no es un peon el que mueve o una captura
    }
    else{
        // Si el movim. es de captura. Resetea el contador moves_fifty
        board.modif_medias(0);
    }
    
    if movim.piece_moving < 0{
        // Incrementa el total de jugadas completas si la pieza que se mueve es negra
        incr = board.moves_full;
        incr += 1;
        board.modif_todas(incr);
    }
    
    // ahora buscamos que tipo de movimiento es y actuar en funcion de eso
    let indice: i16;          // para trabajar con los indices. La resta de indices puede ser negativa
    
    if movim.move_type == defs::ORDINARY_MOVE{
        board.board_array[movim.to_index] = movim.piece_moving;         // activa la casilla objetivo
        board.board_array[movim.from_index] = defs::EMPTY_SQUARE;      // limpia la casilla de origen
        
        // Verifica si hay captura al paso 
        // Si la pieza que se mueve es un peón blanco o negro, 
        // y si se ha movido 2 casillas (la instrucc. std::num::abs, numero.abs() verifica eso) 
        // establece la casilla 'pasada' al índice correcto y salimos de la jugada
        indice =  movim.to_index as i16 - movim.from_index as i16;
        if (movim.piece_moving == defs::W_PAWN || movim.piece_moving == defs::B_PAWN) &&
                        indice.abs() == 32{
                board.en_passant = movim.from_index as i16 + (indice)/2;
        }
    }
    
    else if movim.move_type == defs::SHORT_CASTLE{
        if movim.piece_moving == defs::W_KING{               //# el rey blanco se enroca corto
            board.board_array[7] = defs::EMPTY_SQUARE;       //# se vacia la casilla de la torre
            board.board_array[4] = defs::EMPTY_SQUARE;       //# se vacia la casilla del rey
            board.board_array[6] = defs::W_KING;             //# ponemos el rey blanco
            board.board_array[5] = defs::W_ROOK;             //# ponemos la torre blanca
            board.white_castle = defs::CASTLE_NONE;         //# cambiamos el flag para hacer imposible otro enroque
        }
        else{
            board.board_array[119] = defs::EMPTY_SQUARE;    //# vaciamos la casilla de la torre
            board.board_array[116] = defs::EMPTY_SQUARE;    //# vaciamos la casilla del rey
            board.board_array[118] = defs::B_KING;          //# ponemos el rey
            board.board_array[117] = defs::B_ROOK;          //# ponemos la torre negra
            board.black_castle = defs::CASTLE_NONE;        //# imposibilitamos ulteriores enroques del negro
        }
    }
    
    else if movim.move_type == defs::LONG_CASTLE{
        if movim.piece_moving == defs::W_KING{                   //# el rey blanco se enroca largo
            board.board_array[0] = defs::EMPTY_SQUARE;           //# se vacia la casilla de la torre
            board.board_array[4] = defs::EMPTY_SQUARE;           //# se vacia la casilla del rey
            board.board_array[2] = defs::W_KING;                 //# ponemos el rey blanco
            board.board_array[3] = defs::W_ROOK;                 //# ponemos la torre blanca
            board.white_castle = defs::CASTLE_NONE;             //# imposibilitamos mas enroques
        }
        else{
            board.board_array[112] = defs::EMPTY_SQUARE;         //# vaciamos la casilla de la torre
            board.board_array[116] = defs::EMPTY_SQUARE;         //# vaciamos la casilla del rey
            board.board_array[114] = defs::B_KING;               //# ponemos el rey negro
            board.board_array[115] = defs::B_ROOK;               //# ponemos la torre negra
            board.black_castle = defs::CASTLE_NONE;              //# imposibilitamos mas enroques
        }
    }
    
    else if movim.move_type == defs::EN_PASSANT{
        let idx: usize;
        if board.to_move == -1{
            /*
            # hemos cambiado de lado, y esto significa que es el blanco el que esta moviendo
            # Ya que es una captura al paso, también necesitamos eliminar el peón capturado 
            # que se encuentra una casilla arriba / abajo de la casilla destino. 
            # Si se trata de una captura de peón blanco, borramos la casilla que se encuentra debajo.
            */
            idx = (movim.to_index - 16) as usize;
            board.board_array[idx] = defs::EMPTY_SQUARE;
        }
        else{
            idx = (movim.to_index + 16) as usize;
            board.board_array[idx] = defs::EMPTY_SQUARE
        }
        board.board_array[movim.to_index] = movim.piece_moving;        //# pone la casilla destino
        board.board_array[movim.from_index] = defs::EMPTY_SQUARE;     //# limpia la casilla origen
    }
    
    // capturamos las promociones que es lo unico que nos queda
    else{
        let mut turno: i16;
        if movim.move_type == defs::PROMOTION_QUEEN{
            turno = board.to_move * -1;
            board.board_array[movim.to_index] = defs::W_QUEEN * turno;
        }
        if movim.move_type == defs::PROMOTION_ROOK{
            turno = board.to_move * -1;
            board.board_array[movim.to_index] = defs::W_ROOK * turno;
        }
        if movim.move_type == defs::PROMOTION_BISHOP{
            turno = board.to_move * -1;
            board.board_array[movim.to_index] = defs::W_BISHOP * turno;
        }
        if movim.move_type == defs::PROMOTION_KNIGHT{
            turno = board.to_move * -1;
            board.board_array[movim.to_index] = defs::W_KNIGHT * turno;
        }
            
        board.board_array[movim.from_index] = defs::EMPTY_SQUARE;     //# vaciamos la casilla origen
    }
    
    /*
    # Ahora verificamos los cambios correctos de enroque, algunos de ellos están hechos arriba, 
    # pero debemos atraparlos tanto para promociones de peones como para movimientos normales, 
    # por lo que lo tenemos fuera del interruptor.

    # Si ni el blanco ni el negro pueden enrocarse, no necesitamos verificar los cambios 
    # de enroque y podemos saltar los chequeos
    */
    
    if board.white_castle != defs::CASTLE_NONE || board.black_castle != defs::CASTLE_NONE{
        /*
        # Ahora revisamos las esquinas y las casillas del rey, 
        # si una pieza (torre o rey) "falta" en uno de esos cuadrados, 
        # podemos eliminar los derechos de enroque en consecuencia.

        # Si el movimiento es Ra2-a1 para el negro, 
        # en el siguiente chequeo parecerá que todo está en orden para el enroque, 
        # por lo que el derecho de enroque no se eliminará, 
        # PERO ese derecho de enroque ya debería haberse eliminado cuando 
        # la torre se movió a a2 En primer lugar, no tenemos que preocuparnos por eso.
        */
        if board.board_array[4] != defs::W_KING{         //# El rey blanco no está en 'e1'
            board.white_castle = defs::CASTLE_NONE;     //# Quitamos los derechos de enroque blanco
        }
        
        if board.board_array[116] != defs::B_KING{       //# El rey negro no está en e8
            board.black_castle = defs::CASTLE_NONE;     //# quitamos derechos enroque del negro
        }
            
        if board.board_array[0] != defs::W_ROOK{             //# torre blanca no esta en 'a1'
            if board.white_castle == defs::CASTLE_BOTH{     //# deshabilitamos el enroque largo
                board.white_castle = defs::CASTLE_SHORT;
            }
            else if board.white_castle == defs::CASTLE_LONG{
                board.white_castle = defs::CASTLE_NONE;
            }
        }
        
        if board.board_array[7] != defs::W_ROOK{             //# torre blanca no esta en 'h1'
            if board.white_castle == defs::CASTLE_BOTH{     //# deshabilitamos enroque corto
                board.white_castle = defs::CASTLE_LONG;
            }
            else if board.white_castle == defs::CASTLE_SHORT{
                board.white_castle = defs::CASTLE_NONE;
            }
        }
                
        if board.board_array[112] != defs::B_ROOK{           //# torre negra no está en 'a8'
            if board.black_castle == defs::CASTLE_BOTH{     //# deshabilita enroque largo
                board.black_castle = defs::CASTLE_SHORT;
            }
            else if board.black_castle == defs::CASTLE_LONG{
                board.black_castle = defs::CASTLE_NONE;
            }
        }
                
        if board.board_array[119] != defs::B_ROOK{               //# la torre falta de 'h8'
            if board.black_castle == defs::CASTLE_BOTH{         //# deshabilita el enroque corto
                board.black_castle = defs::CASTLE_LONG;
            }
            else if board.black_castle == defs::CASTLE_SHORT{
                board.black_castle = defs::CASTLE_NONE;
            }
        }
    }
    
    // ahora vamos a comprobar si se ha hecho el movimiento
    //println!("925 -- {}", get_fen(board));
    // retornamos el movim para reutilizar el valor
    movim
}
// FIN de make_movim()



/*
	 *  Deshace la jugada en el tablero
	 *
	 *  @param movim de tipo Movim
*/
fn unmake_movim(board: &mut Tablero, movim: Movim){
    //reseteamos las variables conocidas
    board.en_passant = movim.prev_pos[0];
    board.white_castle = movim.prev_pos[1];
    board.black_castle = movim.prev_pos[2];
    board.moves_fifty = movim.prev_pos[3];
    
    // cambiamos al bando a mover
    let turno: i16 = board.to_move * -1;
    //turno = turno * -1;
    board.to_move = turno;
    
    /*
    # Si el movimiento que estamos recuperando fue una pieza negra, 
    # el movimiento disminuye las moves_full.
    */
    if movim.piece_moving < 0{
        let mut todas: i16 = board.moves_full;
        todas -= 1;
        board.moves_full = todas;
    }
    
    if movim.move_type == defs::SHORT_CASTLE{
        if movim.piece_moving == defs::W_KING{               //# El rey blanco enroca corto
            board.board_array[5] = defs::EMPTY_SQUARE;       //# Vaciamos las casillas del enroque
            board.board_array[6] = defs::EMPTY_SQUARE;       //# .
            board.board_array[4] = defs::W_KING;             //# ponemos de vuelta el rey
            board.board_array[7] = defs::W_ROOK;             //# ponemos de vuelta la torre blanca
        }
        else if movim.piece_moving == defs::B_KING{          //# el rey negro enroca corto
            board.board_array[117] = defs::EMPTY_SQUARE;     //# Vaciamos las casillas del enroque
            board.board_array[118] = defs::EMPTY_SQUARE;     //# .
            board.board_array[116] = defs::B_KING;           //# ponemos de vuelta el rey negro
            board.board_array[119] = defs::B_ROOK;           //# ponemos de vuelta la torre negra
        }
    }
    
    else if movim.move_type == defs::LONG_CASTLE{
        if movim.piece_moving == defs::W_KING{                //# El rey blanco se enroca largo
            board.board_array[2] = defs::EMPTY_SQUARE;       //# Vaciamos las casillas de enroque
            board.board_array[3] = defs::EMPTY_SQUARE;       //# .
            board.board_array[4] = defs::W_KING;             //# Ponemos el rey de vuelta
            board.board_array[0] = defs::W_ROOK;             //# Ponemos la torre de vuelta
        }
        else if movim.piece_moving == defs::B_KING{           //# El negro se enroca largo
            board.board_array[114] = defs::EMPTY_SQUARE;     //# vaciamos la casilla del enroque
            board.board_array[115] = defs::EMPTY_SQUARE;     //# .
            board.board_array[116] = defs::B_KING;           //# Ponemos el rey negro de vuelta
            board.board_array[112] = defs::B_ROOK;           //# Ponemos de vuelta la torre negra
        }
    }
    
    else if movim.move_type == defs::EN_PASSANT {
        
        board.board_array[movim.from_index] = movim.piece_moving;      // Ponemos el peon de vuelta
        board.board_array[movim.to_index] = defs::EMPTY_SQUARE;       // Limpiamos la casilla original
        
        // Ponemos de vuelta un peon negro en la casilla correcta
        // si es el blanco el que está moviendo
        if movim.piece_moving == defs::W_PAWN{
            board.board_array[movim.to_index - 16] = defs::B_PAWN;
        }
        // si no es el blanco, entonces solo queda el negro
        else{
            board.board_array[movim.to_index + 16] = defs::W_PAWN;
        }
    }
            
    else{
        board.board_array[movim.from_index] = movim.piece_moving;      //# Ponemos la pieza de vuelta
        board.board_array[movim.to_index] = movim.capture;            //# La captura mantiene el valor para
        //# numero de la pieza capturada o 0
        //# (EMPTY_SQUARE) si no es una captura
    }
    // ahora vamos a comprobar si se ha hecho el movimiento
    //println!("1014 -- {}", get_fen(board));
}
// FINAL unmake_movim()


// ============ fin de make y unmake_movim ======
// luego generamos los posible movims. legales 

/******************************************/
/** Metodos de generacion de movimientos **/
/****************************+++++++++++++*/

/**
 *  Retorna un vector con todas las jugadas legales posibles
 *
 */
fn generate_moves(board: &mut Tablero) -> Vec<Movim>{
    let mut pseudo_moves: Vec<Movim> = Vec::new();
    generate_pseudo_moves(board, &mut pseudo_moves);
    let legal_moves = filter_moves(board, pseudo_moves);
    // retornamos solo las jugadas legales
    legal_moves
}
// FIN getLegalMoves()




/*
 *  Toma una lista con las jugadas seudo-legales y quita las ilegales
 *  retornando una lista con las jugadas legales
 *
 *  @param board y vector   con la direccion en memoria de las jugadas seudo-legales
 *  @return Vector          retorna las jugadas legales
*/

fn filter_moves(board: &mut Tablero, pseudo_moves: Vec<Movim>) -> Vec<Movim>{
    let mut filtered_moves: Vec<Movim> = Vec::new();
    
    // Comenzamos encontrando el rey en el tablero
    let mut king_square: i16 = -1;
    // el indice para el bucle
    let mut index: i16 = 0;
    
    while index < 120 {
        if (index & 0x88) == 0{
            /*
            # Queremos encontrar el rey del bando que mueve,
            # asi que se multiplica por to_move y encuentra el rey 'blanco'
            */
            if board.board_array[index as usize] * board.to_move == defs::W_KING{
                king_square = index;     //# Recuerda el index del rey
                index = 120;             //# Fin del bucle
            }
        }
        else{
            index += 7;
        }
        index += 1;
    }
    
    if king_square == -1{
        // No se ha encontrado el rey en las seudomoves, 
        // por lo que no podemos filtrar las ilegales
        filtered_moves = pseudo_moves;
        return filtered_moves ;
    }
    
    //# Ahora recorremos cada seudo-jugada
    for i in 0..pseudo_moves.len(){
        let mut current_move = pseudo_moves[i as usize];     // Obtiene un movimiento de la lista
        /*
        # Si el movim. es un enroque tambien necesitamos comprobar el enroque en jaque
        # y las casillas de transito, por tanto comprobamos el tipo de movimiento
        */
        if current_move.move_type == defs::SHORT_CASTLE{
            make_movim(board, current_move);     // Hace la jugada en el tablero
            /*
            # Ahora verificamos si la casilla original del rey o las dos a 
            # la derecha son atacadas.
            # Esto atrapa las tres posibilidades; Enroque bajo jaque, 
            # enroque sobre casilla atacada, y enroque bajo jaque
            #
            # Si ninguno de ellos es atacado podemos agregarlo a movimientos legales.
            */
            if !es_atacada(board, current_move.from_index as i16) &&
                        !es_atacada(board, (current_move.from_index + 1) as i16) &&
                        !es_atacada(board, (current_move.from_index + 2) as i16){
                filtered_moves.push(current_move);
            }
            unmake_movim(board, current_move);     //# deshacemos la jugada para restaurar el tablero
        }
        else if current_move.move_type == defs::LONG_CASTLE{
            make_movim(board, current_move);    //# Hacemos la jugada en el tablero

            // aqui hacemos lo mismo que arriba para SHORT_CASTLE
            if !es_atacada(board, current_move.from_index as i16) &&
                        !es_atacada(board, (current_move.from_index - 1) as i16) &&
                        !es_atacada(board, (current_move.from_index - 2) as i16){
                filtered_moves.push(current_move);
            }
            unmake_movim(board, current_move);      //deshacemos la jugada para restaurar el tablero
        }
        // # capturamos todos los demas tipos de jugadas
        else{   
            make_movim(board, current_move);    //# realizamos el movim. en el tablero
            /*
            # Ya hemos determinado la casilla del rey, pero si el rey se está moviendo, 
            # debemos llamar a isAtattacked con su nuevo casilla y no donde estaba antes.
            */
            if current_move.piece_moving == defs::W_KING ||
                            current_move.piece_moving == defs::B_KING{
                if !es_atacada(board, current_move.to_index as i16){
                    filtered_moves.push(current_move);
                }
            }
            //# Si no es el rey el que está moviendo, usamos dondes estaba determinado antes
            else{
                //# Si el no puede ser atacado añadimos la jugada
                if !es_atacada(board, king_square){
                    filtered_moves.push(current_move);
                }
            }
            unmake_movim(board, current_move);    //# Deshacemos la jugada y restauramos el tablero
        }
    }
    
    // retornamos solo las jugadas validas
    filtered_moves
}




fn es_atacada(board: &mut Tablero, attacked: i16) -> bool{
    let mut attacker: i16 = 0;
    
    while attacker < 120{
        if (attacker & 0x88) == 0{      // Si estamos en las casillas del tablero real
            /*
            # Cambia el color de la pieza en la casilla y lo mantiene en la variable, 
            # esto se hace para hacer el código más claro a continuación.
            
            # Entonces, si el negro se está moviendo y encontramos una reina negra (-2), 
            # la convertimos en una reina blanca (2) multiplicando por to_move = -1 (movimiento del negro). 
            # Ahora solo tenemos que revisar las piezas 'blancas' a continuación.
            */
            let piece_on_square = board.board_array[attacker as usize] * board.to_move;
            
            if defs::ATTACK_ARRAY[(attacked-attacker+128) as usize] == defs::ATTACK_NONE{
                let _xxx = 1;   // no hacemos nada
            }
            
            else if defs::ATTACK_ARRAY[(attacked-attacker+128) as usize] == defs::ATTACK_KQR{
                /*
                // Si la casilla chequeada contiene alguna de las piezas que
                // son capaces de llegar a la casilla atacada.
                //
                // Ya que está a solo una casilla de distancia de la casilla 
                // atacada, podemos concluir con seguridad que la casilla es atacada.
                */
                if piece_on_square == defs::W_KING ||
                        piece_on_square == defs::W_QUEEN ||
                        piece_on_square == defs::W_ROOK{
                    return true;
                }
            }
            
            else if defs::ATTACK_ARRAY[(attacked-attacker+128) as usize] == defs::ATTACK_QR{
                if piece_on_square == defs::W_QUEEN || piece_on_square == defs::W_ROOK{
                    // Si el racorrido (traverse) golpea la casilla atacada
                    // la pieza puede ser atacada.
                    if traverse_delta(board, attacker, attacked){
                            return true;
                    }
                }
            }
            
            else if defs::ATTACK_ARRAY[(attacked-attacker+128) as usize] == defs::ATTACK_KQB_WP{
                if piece_on_square == defs::W_KING ||
                        piece_on_square == defs::W_QUEEN ||
                        piece_on_square == defs::W_BISHOP{
                    return true;
                }
                /*
                # Si es el turno de las blancas para moverse y la pieza es un peón blanco, 
                # puede atacar la casilla.

                # Hacemos esta distinción para no confundir los peones negros y blancos, 
                #ya que solo pueden moverse en una dirección dependiendo del color.
                */
                if board.to_move == 1 && board.board_array[attacker as usize] == defs::W_PAWN{
                    return true;
                }
            }
            
            else if defs::ATTACK_ARRAY[(attacked-attacker+128) as usize] == defs::ATTACK_KQB_BP{
                    if piece_on_square == defs::W_KING ||
                            piece_on_square == defs::W_QUEEN ||
                            piece_on_square == defs::W_BISHOP{
                        return true;
                    }
                    /*
                    #// Si es el turno del negro y la pieza es un peon blanco
                    #// puede atacar la casilla.
                    */
                    if board.to_move == -1 && board.board_array[attacker as usize] == defs::B_PAWN{
                        return true;
                    }
            }
            
            else if defs::ATTACK_ARRAY[(attacked-attacker+128) as usize] == defs::ATTACK_QB{
                if piece_on_square == defs::W_QUEEN || piece_on_square == defs::W_BISHOP{
                    /*
                    #// If the traverse hits the attacked square
                    #// the piece can attack it.
                    */
                    if traverse_delta(board, attacker, attacked){
                        return true;
                    }
                }
            }
            
            else if defs::ATTACK_ARRAY[(attacked-attacker+128) as usize] == defs::ATTACK_N{
                /*
                #// Knights can jump over pieces so we don't have to
                #// check for pieces in the way.
                */
                if piece_on_square == defs::W_KNIGHT{
                    return true;
                }
            }
        }
        
        else{ 
            attacker += 7;          //#; // Not on board, so move to next rank
        }                           //#// +7 since for-loop will add 1 as well
        
        attacker += 1;
    }
    return false                    //#; // If the loop didn't return true, no piece can attack the square
}



/*
 *  Usado por es_atacada() para recorrer el delta de una pieza para
 *  ver si se interpone en el camino de cualquier pieza a la casilla atacada.
 *
 *  @param int attacker     La casilla atacante
 *  @param int attacked     La casilla atacada
 *  @return booleano        True si la pieza puede alcanzar la casilla atacada, False si no puede
*/
fn traverse_delta(board: &mut Tablero, attacker: i16, attacked: i16) -> bool {
    let mut delta_index: i16 = attacker;                                        //# Inicializa desde la primera casilla
    let delta: i16 = defs::DELTA_ARRAY[(attacked - attacker + 128) as usize];   //# encuentra el delta necesitado
    
    loop {
        delta_index += delta; // Añade el delta de la jug. a la casilla siguiente

        // Alcanzamos la casilla atacada, retornamos por tanto true
        if delta_index == attacked{
            return true;
        }
        // Se ha encontrado una pieza en el camino, por lo que retornamos false
        if board.board_array[delta_index as usize] != defs::EMPTY_SQUARE{
            return false;
        }
    }
}





/**
 *  Retorna un vector con los movimientos pseduo-legales
 *
 * @return Vector   Todas las jugadas pseudo-legales sobre el tablero
 */

fn generate_pseudo_moves(board: &mut Tablero, pseudo_moves: &mut Vec<Movim>) {
    let mut index: i16 = 0;
    
    while index < 120 {
        if (index & 0x88) == 0{     //entramos en el tablero real
            /*
            # Verifica la casilla, si el negro es el que está moviendo, 
            # verificamos los valores negativos (* (- 1)), es decir, piezas negras. 
            # Por lo tanto, un -5 (caballo negro) se "transformará" en un caballo blanco por un momento 
            # y se recogerá debajo.
            */
            
            if board.board_array[index as usize] * board.to_move == defs::EMPTY_SQUARE{
                let _xxx = 1;    // no hace nada util no enuentro un pass como en Python
            }
            else if board.board_array[index as usize] * board.to_move == defs::W_QUEEN{
                // necesito pasar el i16 indice
                gen_moves(board, index, defs::QUEEN_DELTA, true, pseudo_moves);
            }
            else if board.board_array[index as usize] * board.to_move == defs::W_ROOK{
                gen_moves(board, index, defs::ROOK_DELTA, true, pseudo_moves);
            }
            else if board.board_array[index as usize] * board.to_move == defs::W_BISHOP{
                gen_moves(board, index, defs::BISHOP_DELTA, true, pseudo_moves);
            }
            else if board.board_array[index as usize] * board.to_move == defs::W_KNIGHT{
                gen_moves(board, index, defs::KNIGHT_DELTA, false, pseudo_moves);
            }
            else if board.board_array[index as usize] * board.to_move == defs::W_KING{
                gen_moves(board, index, defs::KING_DELTA, false, pseudo_moves);
                gen_castling(board, pseudo_moves);
            }
            else if board.board_array[index as usize] * board.to_move == defs::W_PAWN{
                gen_pawn(board, index, pseudo_moves);
            }
        }
        else{
            index += 7;     // fuera del tablero por lo que nos movemos a la fila siguiente
        }
        index += 1;
    }
}
// FIN generate_moves()



/*
 *  Toma un index, un delta, booleano deslizante/no deslizante y la lista pseudoMoves
 *  y rellena la lista con las jugadas pseudo-legales para la pieza
 *  y entonces retorna
 *
 *  @param int index            El indice donde está la pieza
 *  @param int List delta       La delta de la pieza
 *  @param boolean sliding      Deslizante / no-deslizante
 *  @param List pseudoMoves     La lista de pseudoMoves a rellenar
 *  @return List pseudoMoves    La pseudoMoves actualizada
*/

fn gen_moves(board: &mut Tablero, index: i16, delta: [i16;8], sliding: bool, pseudo_moves: &mut Vec<Movim>){
    /* 
    # Registra la casilla al-paso del tablero, los derechos de enroque blanco / negro y 
    # los medios movimientos.
    */
    let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
    for i in 0..8{      //# recurre los 8 deltas válidos
        //# Obtenemos el índice de una casilla a un paso de la casilla 
        //# original usando el delta actual
        let mut delta_index: i16 = index;
        delta_index += delta[i];
        
        while delta[i] != 0{                //# ejecuta el bucle si tenemos un delta válido
            if (delta_index & 0x88) == 0{    //# casilla objetivo esta en el tablero
                
                // 1.- La casilla objetivo está vacía
                if board.board_array[delta_index as usize] == defs::EMPTY_SQUARE{
                    //# Añade la jugada a las pseudoMoves
                    let mut movim = Movim::init(board.board_array[index as usize], index as usize, 
                                delta_index as usize, 0, defs::ORDINARY_MOVE, prev_pos);
                    pseudo_moves.push(movim);
                    //# Si la pieza en movimiento es deslizante, añadimos la siguiente casilla
                    if sliding{
                        delta_index += delta[i];
                    }
                    else{
                        break;       //# Si no es deslizante, paramos el calculo para este delta
                    }
                }
                // 2.- las casilla objetivo no esta vacia
                else{
                    if (board.board_array[index as usize] * board.board_array[delta_index as usize]) < 0 {
                        /*
                        # Si la pieza en movimiento multiplicada por la pieza objetivo es < 0. 
                        # Sabemos que la pieza objetivo es de color opuesto (num positivo + num. negativo)
                        
                        # Agrega el movimiento como captura a pseudoMoves 
                        # (es decir, envía la pieza int como captura)
                        */
                        let mut movim = Movim::init(board.board_array[index as usize], index as usize, 
                                    delta_index as usize, board.board_array[delta_index as usize], 
                                    defs::ORDINARY_MOVE, prev_pos);
                        pseudo_moves.push(movim);
                        break;          //# no podemos ir más allá en esta dirección
                    }
                    else{
                        break;   
                        //# La casilla objetivo contiene una pieza del mismo color por lo que no hay mas movs.
                    }
                }
            }
            else{
                break;      //hemos salido del tablero, asi que dejamos de comprobar en esta direccion
            }
        }
    }
}
// FIN de gen_moves()



/*
 *  Añade todas las jugadas posibles de peon
 *
 *  @param Struct       el tablero
 *  @param int          Index donde está la pieza
 *  @param Vector       la lista de pseudoMoves a rellenarse
*/

fn gen_pawn(board: &mut Tablero, index: i16, pseudo_moves: &mut Vec<Movim>){
    let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
    
    if board.to_move == defs::WHITE_TO_MOVE{
        //# Si la siguiente casilla está vacía movemos alli
        if board.board_array[(index + 16) as usize] == defs::EMPTY_SQUARE{
            if (index-(index%16))/16 != 6{      //# si el peon no esta en la septima fila
                                                //# Añadimos una jugada normal
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 16) as usize, 
                                        0, defs::ORDINARY_MOVE, prev_pos));
            }
            else{       //# Añadimos la promoción si estamos en la septima fila
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 16) as usize, 
                                            0, defs::PROMOTION_QUEEN, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 16) as usize, 
                                            0, defs::PROMOTION_ROOK, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 16) as usize, 
                                            0, defs::PROMOTION_BISHOP, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 16) as usize, 
                                            0, defs::PROMOTION_KNIGHT, prev_pos));
            }
        }
        /*
        # Si la siguiente casilla en diagonal a la izquierda contiene una pieza del bando opuesto, 
        # es posible capturar
        #if self.board_array[(index + 15)] < 0:
        */
        if ((index + 15) & 0x88) == 0 && board.board_array[(index + 15) as usize] < 0{
            // si no está en la septima fila
            if(index-(index%16))/16 != 6{ 
                //# Añadimos una jugada normal
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 15) as usize, 
                        board.board_array[(index + 15) as usize], defs::ORDINARY_MOVE, prev_pos))
            }
            else{       // Añade promociona si esta en la septima fila
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 15) as usize, 
                        board.board_array[(index + 15) as usize], defs::PROMOTION_QUEEN, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 15) as usize, 
                        board.board_array[(index + 15) as usize], defs::PROMOTION_ROOK, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 15) as usize, 
                        board.board_array[(index + 15) as usize], defs::PROMOTION_BISHOP, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 15) as usize, 
                        board.board_array[(index + 15) as usize], defs::PROMOTION_KNIGHT,prev_pos));
            }
        }
        /*
        # Si el siguiente cuadrado en diagonal a la derecha contiene una pieza lateral opuesta, 
        # es posible capturar
        #if self.board_array[(index + 17)] < 0:
        */
        if ((index + 17) & 0x88) == 0 && board.board_array[(index + 17) as usize] < 0{
            // si no esta en fila 7
            if (index-(index%16))/16 != 6{
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 17) as usize, 
                                board.board_array[(index + 17) as usize], defs::ORDINARY_MOVE, prev_pos));
            }
            else{
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 17) as usize, 
                                board.board_array[(index + 17) as usize], defs::PROMOTION_QUEEN, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 17) as usize, 
                                board.board_array[(index + 17) as usize], defs::PROMOTION_ROOK, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 17) as usize, 
                                board.board_array[(index + 17) as usize], defs::PROMOTION_BISHOP, prev_pos));
                pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, (index + 17) as usize, 
                                board.board_array[(index + 17) as usize], defs::PROMOTION_KNIGHT, prev_pos));
            }
        }
        /*
        # Si el peón está en su casilla de origen (rango 2) es posible mover dos cuadrados 
        # si ambos cuadrados de delante están vacíos
        */
        if ((index-(index%16))/16 == 1) && (board.board_array[(index + 16) as usize] == defs::EMPTY_SQUARE) && 
                                            (board.board_array[(index + 32) as usize] == defs::EMPTY_SQUARE){
            pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize, 
                    (index + 32) as usize, 0, defs::ORDINARY_MOVE, prev_pos));
        }
        
        //# Si hay una casilla al paso y el peón puede alcanzarla, podemos añadir al-paso.
        if board.en_passant != -1 && (index+17 == board.en_passant || index+15 == board.en_passant){
            pseudo_moves.push(Movim::init(defs::W_PAWN, index as usize,
                    board.en_passant as usize, defs::B_PAWN, defs::EN_PASSANT, prev_pos));
        }
    }
    
    // ahora mueve el negro
    else{
        // Si la casilla está vacía movemos alli
        if board.board_array[(index - 16) as usize] == defs::EMPTY_SQUARE{ 
            // si el peon no está en la segunda fila
            if (index-(index%16))/16 != 1{ 
                //# Añade una jugada normal de peon
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 16) as usize, 
                                0, defs::ORDINARY_MOVE, prev_pos))
            }
            else{       // Añade promoción si esta en la fila 7
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 16) as usize, 
                                    0, defs::PROMOTION_QUEEN, prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 16) as usize, 
                                    0, defs::PROMOTION_ROOK, prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 16) as usize, 
                                    0, defs::PROMOTION_BISHOP, prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 16) as usize, 
                                    0, defs::PROMOTION_KNIGHT, prev_pos));
            }
        }
        /*
        # Si el siguiente cuadrado en diagonal a la izquierda contiene una pieza 
        # del bando opuesto, es posible capturar
        */
        if ((index - 15) & 0x88) == 0 && board.board_array[(index - 15) as usize] > 0{
            if (index-(index%16))/16 != 1{     //# si no esta en la fila 2
                //# añade una jugada normal
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 15) as usize, 
                                board.board_array[(index - 15) as usize], defs::ORDINARY_MOVE, prev_pos));
            }
            else{       //# añadimos promocion si estamos en la fila 2
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 15) as usize, 
                                board.board_array[(index - 15) as usize], defs::PROMOTION_QUEEN, prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 15) as usize, 
                                board.board_array[(index - 15) as usize], defs::PROMOTION_ROOK, prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 15) as usize, 
                                board.board_array[(index - 15) as usize], defs::PROMOTION_BISHOP,prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 15) as usize, 
                                board.board_array[(index - 15) as usize], defs::PROMOTION_KNIGHT, prev_pos));
            }
        }
        /*
        # Si la siguiente casilla diagonal contiene una pieza del
        # bando contrario, es posible capturar
        #if self.board_array[(index - 17)] > 0:
        */
        if ((index - 17) & 0x88) == 0 && board.board_array[(index - 17) as usize] > 0{
            if(index-(index%16))/16 != 1{       //# si no está en la fila 2
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 17) as usize, 
                                    board.board_array[(index - 17) as usize], defs::ORDINARY_MOVE, prev_pos));
            }
            else{
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 17) as usize, 
                                    board.board_array[(index - 17) as usize], defs::PROMOTION_QUEEN, prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 17) as usize, 
                                    board.board_array[(index - 17) as usize], defs::PROMOTION_ROOK, prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 17) as usize, 
                                    board.board_array[(index - 17) as usize], defs::PROMOTION_BISHOP, prev_pos));
                pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 17) as usize, 
                                    board.board_array[(index - 17) as usize], defs::PROMOTION_KNIGHT, prev_pos));
            }
        }
        /*
        # Si el peon está en su casilla origen (fila 7)
        # es posible mover dos casillas siambas están vacias al frente
        */
        if ((index-(index%16))/16 == 6) && (board.board_array[(index - 16) as usize] == defs::EMPTY_SQUARE) &&
                        (board.board_array[(index - 32) as usize] == defs::EMPTY_SQUARE){
            pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, (index - 32) as usize, 0, 
                                defs::ORDINARY_MOVE, prev_pos));
        }
            
        //# Si hay una casilla al paso y el peón puede lograrla
        //# podemos añadir al_paso/en_passant
        if board.en_passant != -1 && (index-17 == board.en_passant || index-15 == board.en_passant){
            pseudo_moves.push(Movim::init(defs::B_PAWN, index as usize, board.en_passant as usize, 
                    defs::W_PAWN, defs::EN_PASSANT, prev_pos));
        }
    }
}




/*
 *  Comprueba si el enroque está disponible para el rey y añade la jugada si es asi
 *
 *  @param Struct   - El tablero
 *  @param Vector  - La lista de pseudomoves para rellenarse
*/
fn gen_castling(board: &mut Tablero, pseudo_moves: &mut Vec<Movim>){

    // # el rey blanco está moviendo
    if board.to_move == defs::WHITE_TO_MOVE{
    
        if board.white_castle == defs::CASTLE_NONE{
            // no hacemos nada
            let _xxx = 1;
        }
        
        else if board.white_castle == defs::CASTLE_SHORT{
            // Si las casillas entre el rey y la torre están vacías, 
            // añadir enroque corto
            if (board.board_array[5] == defs::EMPTY_SQUARE) &&
                        (board.board_array[6] == defs::EMPTY_SQUARE){
                let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
                pseudo_moves.push(Movim::init(defs::W_KING, 4 as usize, 6 as usize, 0, 
                                    defs::SHORT_CASTLE, prev_pos));
            }
        }
        
        else if board.white_castle == defs::CASTLE_LONG{
            // Si las casillas entre el rey y la torre estan vacias, 
            // añadir enroque largo
            if (board.board_array[1] == defs::EMPTY_SQUARE) &&
                        (board.board_array[2] == defs::EMPTY_SQUARE) &&
                        (board.board_array[3] == defs::EMPTY_SQUARE){
                let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
                pseudo_moves.push(Movim::init(defs::W_KING, 4 as usize, 2 as usize, 0, 
                                    defs::LONG_CASTLE, prev_pos));
            }
        }
        
        else{       // puede enrocarse en ambos lados
            // Si las casillas entre el rey la torre estan vacias, 
            // añadir enroque corto
            if(board.board_array[5] == defs::EMPTY_SQUARE) &&
                        (board.board_array[6] == defs::EMPTY_SQUARE){
                let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
                pseudo_moves.push(Movim::init(defs::W_KING, 4 as usize, 6 as usize, 0, 
                                    defs::SHORT_CASTLE, prev_pos));
            }
            // Si las casillas entre el rey la torre estan vacias, 
            // añadir enroque largo
            if (board.board_array[1] == defs::EMPTY_SQUARE) &&
                            (board.board_array[2] == defs::EMPTY_SQUARE) &&
                            (board.board_array[3] == defs::EMPTY_SQUARE){
                let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
                pseudo_moves.push(Movim::init(defs::W_KING, 4 as usize, 2 as usize, 0, 
                                    defs::LONG_CASTLE, prev_pos));
            }
        }
    }
    
    // es el rey negro el que está moviendo
    else if board.to_move == defs::BLACK_TO_MOVE{
        if board.black_castle == defs::CASTLE_NONE{ 
            // no hay enroque. No hacemos nada
            let _xxx = 1;
        }
        
        else if board.black_castle == defs::CASTLE_SHORT{
            // si las casillas entre el rey la torre estan vacias, 
            // añade enroque corto
            if (board.board_array[117] == defs::EMPTY_SQUARE) &&
                        (board.board_array[118] == defs::EMPTY_SQUARE){
                let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
                pseudo_moves.push(Movim::init(defs::B_KING, 116 as usize, 118 as usize, 0, 
                                    defs::SHORT_CASTLE, prev_pos));
            }
        }
        
        else if board.black_castle == defs::CASTLE_LONG{
            // Si las casillas entre el rey la torre estan vacias, 
            // añadimos enroque largo
            if (board.board_array[115] == defs::EMPTY_SQUARE) &&
                        (board.board_array[114] == defs::EMPTY_SQUARE) &&
                        (board.board_array[113] == defs::EMPTY_SQUARE){
                let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
                pseudo_moves.push(Movim::init(defs::B_KING, 116 as usize,114 as usize, 0, 
                                    defs::LONG_CASTLE, prev_pos));
            }
        }
        
        else{           // se puede enrocar en ambos bandos
            // Si las casillas entre el rey y la torre estan vacias, 
            // se añade el enroque corto
            if (board.board_array[117] == defs::EMPTY_SQUARE) &&
                        (board.board_array[118] == defs::EMPTY_SQUARE){
                let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
                pseudo_moves.push(Movim::init(defs::B_KING, 116 as usize, 118 as usize, 0, 
                                    defs::SHORT_CASTLE, prev_pos));
            }
            
            // Si las casillas entre torre y rey estan vacias, 
            // añadir enroque largo
            if (board.board_array[115] == defs::EMPTY_SQUARE) &&
                        (board.board_array[114] == defs::EMPTY_SQUARE) &&
                        (board.board_array[113] == defs::EMPTY_SQUARE){
                let prev_pos = [board.en_passant, board.white_castle, board.black_castle, board.moves_fifty];
                pseudo_moves.push(Movim::init(defs::B_KING, 116 as usize, 114 as usize, 0, 
                                    defs::LONG_CASTLE, prev_pos));
            }
        }
    }
    
}



/*
    # ================================================
    # convierte la jugada a formato uci
    # ================================================
*/
fn crea_uci(movim: Movim) -> String{
    let mut uci = "".to_string();
    /*
    if (movim.from_index)%16 == 0 { uci.push_str("a") }
    else if (movim.from_index)%16 == 1 { uci.push_str("b") }
    else if (movim.from_index)%16 == 2 { uci.push_str("c") }
    */
    
    let columnafrom = movim.from_index % 16;
    match columnafrom {
        0 => uci.push_str("a"),
        1 => uci.push_str("b"),
        2 => uci.push_str("c"),
        3 => uci.push_str("d"),
        4 => uci.push_str("e"),
        5 => uci.push_str("f"),
        6 => uci.push_str("g"),
        7 => uci.push_str("h"),
        _ => (),    // no hace nada
    };
    
    let filafrom = (movim.from_index-(movim.from_index%16))/16;
    match filafrom {
        0 => uci.push_str("1"),
        1 => uci.push_str("2"),
        2 => uci.push_str("3"),
        3 => uci.push_str("4"),
        4 => uci.push_str("5"),
        5 => uci.push_str("6"),
        6 => uci.push_str("7"),
        7 => uci.push_str("8"),
        _ => (),
    };
    
    let columnato = movim.to_index%16;
    match columnato {
        0 => uci.push_str("a"),
        1 => uci.push_str("b"),
        2 => uci.push_str("c"),
        3 => uci.push_str("d"),
        4 => uci.push_str("e"),
        5 => uci.push_str("f"),
        6 => uci.push_str("g"),
        7 => uci.push_str("h"),
        _ => (), 
    };
    
    let filato = (movim.to_index-(movim.to_index%16))/16;
    match filato {
        0 => uci.push_str("1"),
        1 => uci.push_str("2"),
        2 => uci.push_str("3"),
        3 => uci.push_str("4"),
        4 => uci.push_str("5"),
        5 => uci.push_str("6"),
        6 => uci.push_str("7"),
        7 => uci.push_str("8"),
        _ => (),
    };
    
    let movimtipo = movim.move_type;
    match movimtipo {
        defs::PROMOTION_QUEEN  => uci.push_str("q"),
        defs::PROMOTION_ROOK   => uci.push_str("r"),
        defs::PROMOTION_BISHOP => uci.push_str("b"),
        defs::PROMOTION_KNIGHT => uci.push_str("n"),
        _ => (),
    };
    
    uci
}



/*
    # ================================================= 
    # convrtir SAN a la estructura interna y
    # validar la jugada de la partida
    # =================================================
*/

// quita los decoradores de la cadena SAN
fn stripped_san(san_vestida: String) -> String{
    let san = san_vestida;
    let jug = san.replace('=',"");
    let re = Regex::new(r"[+#]?[?!]*$").unwrap();
    let jug1 = re.replace_all(&jug, "");
    
    jug1.to_string()
}



fn move_from_san(board: &mut Tablero, san: &str) -> Movim{
    let _piezas = "RNBQK";
    let _columnas = "abcdefgh".to_string();
    let _filas = "12345678";
    let _enroques = ("O-O", "O-O-O");
    
    let mut _pieza: i16 = 0;
    let mut _desde: i16 = 0;
    let mut _hasta: i16 = 0;
    let mut _captura: i16 = 0;
    let mut _tipo_movim = 0;
    let mut origen_desambiguado: i16 = -1;
    
    
    let legales = generate_moves(board);
    
    // ahora vamos a examinar la SAN y averiguar todos los datos
    
    let clean_move = stripped_san(san.to_string());
    
    //let mut piece = "";
    let mut _from = ""; 
    //let mut to = "";
    let promotion: i16;     // = 0;
    let coinci_re: bool;    // = false;
    
    /*
    matches.get(0)  --> es la clean_move -- p.ej Rg3f4Q
    matches.get(1)  --> es la pieza que se mueve --> en este caso es "R"
    matches.get(2)  --> es la columna desde --> en este caso "g"
    matches.get(3)  --> es la fila desde --> en este caso "3"
    matches.get(4)  --> es la casilla destino --> en este caso "f4"
    matches.get(5)  --> es la pieza que se corona --> en este caso "Q"
    matches.get(6)  --> será siempre None, porque he limpiado la jugada
    */
    
    let re = Regex::new(r"^([NBKRQ])?([a-h])?([1-8])?x?([a-h][1-8])(=?[nbrqkNBRQK])?(\\+|#)?$").unwrap();
    
    let coincide = re.captures(&clean_move);  //.unwrap();
    match coincide{
        Some(_n) => coinci_re = true,
        None => coinci_re = false,
    }
    if !coinci_re {
        if clean_move == _enroques.0 || clean_move == _enroques.1 {
            if board.to_move == 1{
                _pieza = 1;
                if clean_move == "O-O"{
                    _desde = 4;
                    _hasta = 6;
                }
                else if clean_move == "O-O-O"{
                    _desde = 4;
                    _hasta = 2;
                }
            }
            else{       //juega el negro
                _pieza = -1;
                if clean_move == "O-O"{
                    _desde = 116;
                    _hasta = 118;
                }
                else if clean_move == "O-O-O"{
                    _desde = 116;
                    _hasta = 114;
                }
            }
            
            for k in legales{
                if k.piece_moving == _pieza &&
                        k.from_index == _desde as usize &&
                        k.to_index == _hasta as usize{
                    return k;
                }
            }
        }
        else{
            return Movim::init(0,0,0,0,0,[0,0,0,0]);
        }
    }
    
    else if coinci_re{
        // ha encontrado algo el regex
        let coincide1 = re.captures(&clean_move).unwrap();
        // una variable de control
        let mut retorno: bool;      // = true;
        // primero a ver si existe la casilla destino
        let mut valor: &str = "";
        
        match coincide1.get(4){
            Some(_n) => {
                        valor = coincide1.get(4).unwrap().as_str();
                        
                        //ahora vamos a comprobar que la casilla existe. Convertimos en un vector
                        let char_vec: Vec<char> = valor.chars().collect();
                        let _col = _columnas.find(char_vec[0]);
                        let _fil = _filas.find(char_vec[1]);
                        if _fil == None || _col == None{
                            retorno = false;
                        }
                        else{
                            let index: i16 = defs::SQUARE_NAMES.iter().position(|&r| r == valor).unwrap() as i16;
                            if index >= 0{
                                _hasta = defs::CASILLAS_VALOR[index as usize].indice;
                                retorno = true;
                            }
                            else {
                                retorno = false;
                            }
                        }
                       },
            None => {
                        retorno = false;
                    },
        };
        
        if !retorno{
            return Movim::init(0,0,0,0,0,[0,0,0,0]);
        }
        
        // la pieza que se mueve
        match coincide1.get(1){
            Some(_n) => {
                            let piece_type = coincide1.get(1).unwrap().as_str();
                            // verificamos que la pieza existe
                            let encontrada = _piezas.find(piece_type);
                            
                            if encontrada == None{
                                retorno = false;
                            }
                            else{
                                if board.to_move == defs::WHITE_TO_MOVE{
                                    if piece_type == "N"{
                                        _pieza = 5;
                                    }
                                    else if piece_type == "B"{
                                        _pieza = 4;
                                    }
                                    else if piece_type == "R"{
                                        _pieza = 3;
                                    }
                                    else if piece_type == "Q"{
                                        _pieza = 2;
                                    }
                                    else if piece_type == "K"{
                                        _pieza = 1;
                                    }
                                }
                                else{
                                    if piece_type == "N"{
                                        _pieza = -5;
                                    }
                                    else if piece_type == "B"{
                                        _pieza = -4;
                                    }
                                    else if piece_type == "R"{
                                        _pieza = -3;
                                    }
                                    else if piece_type == "Q"{
                                        _pieza = -2;
                                    }
                                    else if piece_type == "K"{
                                        _pieza = -1;
                                    }
                                }
                                retorno = true;
                            }
                        },
            None => { if board.to_move == defs::WHITE_TO_MOVE{
                            _pieza = 6;
                        }
                        else{
                            _pieza = -6
                        }
                        retorno = true;
                    }
        }
        
        if !retorno{
            return Movim::init(0,0,0,0,0,[0,0,0,0]);
        }
        
        // ya tenemos la pieza y la casilla de destino. Nos falta la casilla origen
        // vamos a dfinir el array desplazamiento de forma implicita
        
        let mut desplazamiento = defs::PAWN_DELTA;
        
        if _pieza.abs() == 6{       // o _pieza == -6
            desplazamiento = defs::PAWN_DELTA;
        }
        else if _pieza.abs() == 5{   
            desplazamiento = defs::KNIGHT_DELTA;
        }
        else if _pieza.abs() == 4{
            desplazamiento = defs::BISHOP_DELTA;
        }
        else if _pieza.abs() == 3{
            desplazamiento = defs::ROOK_DELTA;
        }
        else if _pieza.abs() == 2{
            desplazamiento = defs::QUEEN_DELTA;
        }
        else if _pieza.abs() == 1{
            desplazamiento = defs::KING_DELTA;
        }
        
        let mut posible_origen: Vec<i16> = Vec::new();
        
        // calculamos las posibles casillas desde donde puede haber movido la pieza
        for j in 0..desplazamiento.len(){
            let mut orig = 0;
            // la variable valor se ha definido arriba en el compares(4)
            // peones
            // el desplazamiento de peones depende
            // si es el blanco se resta, si es negro se suma
            if _pieza.abs() == 6 && desplazamiento[j] != 0{
                let index: usize = defs::SQUARE_NAMES.iter().position(|&r| r == valor).unwrap();
                if board.to_move == 1{        // blancas mueven
                    orig = defs::CASILLAS_VALOR[index].indice - desplazamiento[j];
                    if orig & 0x88 == 0 {
                        if board.board_array[orig as usize] != 6 {
                            continue; 
                        }
                    }
                    else { continue; }
                }
                else{
                    orig = defs::CASILLAS_VALOR[index].indice + desplazamiento[j];
                    if orig & 0x88 == 0 {
                        if board.board_array[orig as usize] != -6 { 
                            continue; 
                        }
                    }
                    else { continue;}
                }
            }
            
            // caballo o rey
            if (_pieza.abs() == 5 || _pieza.abs() == 1) && desplazamiento[j] != 0 {
                let index: usize = defs::SQUARE_NAMES.iter().position(|&r| r == valor).unwrap();
                orig = desplazamiento[j] + defs::CASILLAS_VALOR[index].indice;
                if orig & 0x88 == 0 {
                    if board.board_array[orig as usize] == 0 { continue; }
                }
                else { continue; }
            }
            
            //solo para las piezas no deslizantes
            if orig & 0x88 == 0 && (_pieza.abs() == 6 ||
                        _pieza.abs() == 5 || _pieza.abs() == 1){
                posible_origen.push(orig);
            }
            
            // ahora las piezas deslizantes; Alfil, Dama, Torre
            if (_pieza.abs() == 4 || _pieza.abs() == 3 || 
                        _pieza.abs() == 2) && desplazamiento[j] != 0{
                let index: usize = defs::SQUARE_NAMES.iter().position(|&r| r == valor).unwrap();
                let mut acumula_desp = desplazamiento[j] + defs::CASILLAS_VALOR[index].indice;
                
                for _contador in 0..desplazamiento.len(){     //# es max. 8
                    if acumula_desp & 0x88 == 0{
                        if es_atacada(board, defs::CASILLAS_VALOR[index].indice) {
                            //# traverseDelta retorna True si la casilla destino/origen está libre
                            //# si hay una pieza retorna False
                            posible_origen.push(acumula_desp);
                        }
                    }
                    acumula_desp += desplazamiento[j];
                }
            }
        }
        
        //# necesitamos un posible desambiguador. Nc7b4 o Nc2b4, N3e2 etc...
        //# group(2) es la columna, group(3) es la fila
        
        let mut confusa = false;
        
        //desambiguamos
        if coincide1.get(2) != None || coincide1.get(3) != None{
            let col_char: &str;     // = "";
            let fila_char: &str;    // = "";
            match coincide1.get(2){
                Some(_n) => col_char = coincide1.get(2).unwrap().as_str(),
                None => col_char = "None",
            }
            match coincide1.get(3){
                Some(_n) => fila_char = coincide1.get(3).unwrap().as_str(),
                None => fila_char = "None",
            }
                 
            let mut columna: Vec<i16>;  // = Vec::new();
            let mut fila: Vec<i16>;     // = Vec::new();
            let tupla = desambigua(col_char, fila_char);
            columna = tupla.0;
            fila = tupla.1;
            if columna.len() > 0 && fila.len() > 0 {
                for i in 0..columna.len(){
                    for j in 0..fila.len(){
                        if fila[j] == columna[i]{
                            origen_desambiguado = fila[j];
                        }
                    }
                }
            }
            else {
                if columna.len() > 0{
                    // todo esto es para cuando se toma con un peon
                    // cuando hay captura desde la columna adyacente
                    if _pieza.abs() == 6 {
                        let ind_col = defs::COLUMNAS.iter().position(|&r| r.col == col_char).unwrap();
                        if ind_col == 0 {
                            let nuevo_arr = defs::COLUMNAS[ind_col+1];
                            for i in 0..nuevo_arr.inds.len(){
                                columna.push(nuevo_arr.inds[i]);
                            }
                        }
                        else if ind_col == 7 {
                            let nuevo_arr = defs::COLUMNAS[ind_col-1];
                            for i in 0..nuevo_arr.inds.len(){
                                columna.push(nuevo_arr.inds[i]);
                            }
                        }
                        else {
                            let mut nuevo_arr = defs::COLUMNAS[ind_col+1];
                            for i in 0..nuevo_arr.inds.len(){
                                columna.push(nuevo_arr.inds[i]);
                            }
                            nuevo_arr = defs::COLUMNAS[ind_col-1];
                            for i in 0..nuevo_arr.inds.len(){
                                columna.push(nuevo_arr.inds[i]);
                            }
                        }
                    }
                    if fila.len() > 0{
                        let fil = fila_char.parse::<i16>().unwrap() - 1;    //# la fila/columna 1 es el indice 0
                        origen_desambiguado = columna[fil as usize];        //# es un entero
                    }
                    else{
                        let mut _coinci: bool = false;
                        for x in 0..columna.len(){
                            if _coinci { break; }
                            if board.board_array[columna[x] as usize] == _pieza{
                                for y in 0..posible_origen.len(){
                                    if columna[x] == posible_origen[y]{
                                        origen_desambiguado = columna[x] as i16;
                                        _coinci = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                
                if fila.len() > 0{
                    if columna.len() > 0{
                        let col = col_char.parse::<i16>().unwrap() - 1;    //# la columna/fila 1 es el indice 0
                        origen_desambiguado = fila[col as usize];          //# es un entero
                    }
                    else{
                        for x in 0..fila.len(){
                            if board.board_array[fila[x] as usize] == _pieza{
                                origen_desambiguado = fila[x] as i16;
                                break;
                            }
                        }
                    }
                }
                
                /*
                if fila.len() < 1 && columna.len() > 0{
                    //#origen_desambiguado = fila
                    for x in columna{
                        if board.board_array[x as usize] == _pieza{
                            // aqui hay un problema cuando hay peones doblados...
                            origen_desambiguado = x;
                            break;
                        }
                    }
                }
                */
            }
            confusa = true;
        }
        
        // ahora, por ultimo, vamos con la posible promocion
        /*
        Las promociones las tenemos en el modulo defs
        PROMOTION_QUEEN: i16 = 4;
        PROMOTION_ROOK: i16 = 5;
        PROMOTION_BISHOP: i16 = 6;
        PROMOTION_KNIGHT: i16 = 7;
        */
        
        let prom_san: &str;
        
        match coincide1.get(5){
            Some(_n) => prom_san = coincide1.get(5).unwrap().as_str(),
            None => prom_san = "None",
        }
        
        if prom_san == "None"{
            promotion = 0;
        }
        else{
            if prom_san == "Q"{
                promotion = defs::PROMOTION_QUEEN;
            }
            else if prom_san == "R"{
                promotion = defs::PROMOTION_ROOK;
            }
            else if prom_san == "B"{
                promotion = defs::PROMOTION_BISHOP;
            }
            else if prom_san == "N"{
                promotion = defs::PROMOTION_KNIGHT;
            }
            else{
                // no coincide con ninguna promocion
                return Movim::init(0,0,0,0,0,[0,0,0,0]);
                
            }
        }
        
        // construimos los posibles dict de la jugada san
        for origen in posible_origen{
            for k in 0..legales.len(){
                if legales[k].piece_moving == _pieza &&
                        legales[k].from_index == origen as usize &&
                        legales[k].to_index == _hasta as usize{
                    if confusa {
                        if origen == origen_desambiguado{
                            if promotion < defs::PROMOTION_QUEEN{       // no hay promocion de peon
                                return legales[k];
                            }
                            else if legales[k].move_type == promotion{
                                return legales[k];
                            }
                        }
                        else{
                            continue;
                        }
                    }
                    else{
                        if legales[k].move_type == promotion{
                            return legales[k];
                        }
                    }
                        
                }
            }
        }
    }
    
    // si hemos llegado aqui el san es erroneo
    Movim::init(0,0,0,0,0,[0,0,0,0])
}





fn desambigua(grupo2: &str , grupo3: &str) -> (Vec<i16>, Vec<i16>){
    let mut columna: Vec<i16> = Vec::new();
    let mut fila: Vec<i16> = Vec::new();
    
    if grupo2 != "None"{
        for i in 0..defs::COLUMNAS.len(){
            if defs::COLUMNAS[i].col == grupo2{
                let col_posibles = defs::COLUMNAS[i].inds;
                for j in 0..col_posibles.len(){
                    columna.push(col_posibles[j]);
                }
                //columna.extend(defs::COLUMNAS[i].inds);
                break;
            }
        }
    }
    if grupo3 != "None"{
        for i in 0..defs::FILAS.len(){
            // convierto la fila "3" a entero
            if defs::FILAS[i].fila == grupo3.parse::<i16>().unwrap(){
                let fil_posibles = defs::FILAS[i].inds;
                for j in 0..fil_posibles.len(){
                    fila.push(fil_posibles[j]);
                }
                //columna.extend(defs::FILAS[i].inds);
                break;
            }
        }
    }
    
    return (columna, fila);
}



pub fn mueve_san(board: &mut Tablero, san: &str) -> (String, String, String){
    let jugada = move_from_san(board, san);
    let uci = crea_uci(jugada);
    let turno: &str; // = "w";
    if board.to_move == 1{
        turno = "w";
    }
    else { turno = "b"; }
    
    if jugada.piece_moving != 0{
        make_movim(board, jugada);
        // simplemente retornamos una tupla con SAN y UCI y el turno
        return (san.to_string(), uci.to_string(), turno.to_string());
    }
    
    // si hemos llegado hasta aqui es que el movimiento es erroneo
    ("None".to_string(), "None".to_string(), "None".to_string())
    
}