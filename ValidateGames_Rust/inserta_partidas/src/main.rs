extern crate rusqlite;
extern crate regex;
extern crate threadpool;
extern crate num_cpus;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use regex::Regex;
use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};
use std::thread;
use std::io::Write;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::time::Instant;
use threadpool::ThreadPool;
use std::collections::HashMap;
use std::sync::mpsc;
use std::env;


mod ajedrez;


#[derive(Clone, Serialize, Deserialize)]
struct MoveT {
    idx_jug: String,
    san: String,
    uci: String,
    num_jug: String,
    turno: String, 
    fen: String,
    nag: String,
    comen: String,
    sub_var: Vec<usize>,        //las eventuales variantes que cuelgen de este movim.
    profundidad: String,        //esto es a la hora de imprimir (margenes, saltos de linea, etc...)
    
}

impl MoveT {
    pub fn crea_jugada(elem1: String, elem2: String,
                        elem3: String, elem4: String,
                        elem5: String, elem6: String,
                        elem7: String, elem8: String,
                        elem9: String) -> MoveT
    {
        MoveT {
            idx_jug: elem1,
            san: elem2,
            uci: elem3,
            num_jug: elem4,
            turno: elem5,
            fen: elem6,
            nag: elem7,
            comen: elem8,
            sub_var: Vec::new(),
            profundidad: elem9,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct PartiT {
    cabecera : Vec<(String)>,           // serán los campos de RegistrobD
    pgn : String,
    movims: Vec<String>,                // es simplemente el array de las jugadas solamente
    arr_partida : Vec<Vec<MoveT>>,      // es toda la partida. Cada movim. es del tipo moveT
    partida_json: String,
}

impl PartiT {
    pub fn init() -> PartiT{
        PartiT{
            cabecera: Vec::new(),
            pgn : String::new(),
            movims : Vec::new(),
            arr_partida: Vec::new(),
            partida_json: String::new(),
        }
    }
    
    fn cambia_pgn(&mut self, cadena: String) {
        self.pgn = cadena;
    }
    fn cambia_cabecera(&mut self, campo: String) {
        self.cabecera.push(campo);
    }
}


struct RegistroBD {
    num_partida : i64,
    evento : String,
    r_site : String,
    r_date : String,
    r_round : String,
    r_white : String,
    r_black : String,
    r_result : String,
    r_eco : String,
    r_whiteelo : String,
    r_blackelo : String,
    r_fen : String,
    r_pgn : String,
    r_validada : String,
    r_indexado : i8,
    r_borrado : i8,
}


#[derive(Serialize, Deserialize)]
struct ExportaJSON {
    cabecera: Vec<String>,
    partida : Vec<Vec<MoveT>>,
}



fn main() {
    let args: Vec<String> = env::args().collect();
    
    crea_tabla();
    acepta_fichero(&args[1]);
    exporta_partidas();
}



fn estruct_partida() -> &'static str{
    let estructura = "
        CREATE TABLE IF NOT EXISTS tb_partidas (
            num_partida     INTEGER PRIMARY KEY,        -- numero de partida autoincrementado
            evento          TEXT,                       -- nombre del torneo
            sitio           TEXT,                       -- ciudad de la partida
            fecha           TEXT,                       -- fecha de la partida AAAA.MM.DD
            ronda           TEXT,                       -- ronda del torneo
            blancas         TEXT,                       -- jugador con blancas
            negras          TEXT,                       -- nombre jugador con negras
            resultado       TEXT,                       -- 1-0, 0-1, 1/2-1/2, *
            ECO             TEXT,
            elo_blancas     TEXT,
            elo_negras      TEXT,
            FEN             TEXT,
            pgn             TEXT,                       -- es la propia partida en formato pgn
            html            TEXT,                       -- no es html es el formato interno. nombre por convencion
            indexado        INTEGER,                    -- 0=false; 1=true; si han sido, o no, indexados las FEN y movims.
            borrado         INTEGER                     -- 0=false; 1=true; para marcarla como inactiva
        )
    ";
    
    estructura
}

fn crea_tabla() {
    let tabla = estruct_partida();
    
    let idx1 = "CREATE INDEX IF NOT EXISTS idx_partidas_blancas ON tb_partidas (blancas)";
    let idx2 = "CREATE INDEX IF NOT EXISTS idx_partidas_negras ON tb_partidas (negras)";
    let idx3 = "CREATE INDEX IF NOT EXISTS idx_partidas_ECO ON tb_partidas (ECO)";
    
    let conn = Connection::open("sqlite_database/bd_ajedrez.db").unwrap();
    let _res = conn.execute(tabla, NO_PARAMS).unwrap();
    
    conn.execute(idx1, NO_PARAMS).unwrap();
    conn.execute(idx2, NO_PARAMS).unwrap();
    conn.execute(idx3, NO_PARAMS).unwrap();
    
    conn.close().expect("conexion no cerrada");
}


fn acepta_fichero(nombre_fich: &str) {
    let antes = Instant::now();
    let fichero = nombre_fich;   //"pgns/Comentadas.pgn";
    let fich_valido: bool = abre_fichero(fichero);
    if !fich_valido {
        println!("El nombre / ruta del fichero no es válido");
    }
    else {
        println!("El fichero es válido");
    }
    let ahora = Instant::now();
    println!("189 -- Tiempo transcurrido en segundos: {:?}, ", (ahora - antes)/1000);
    println!("190 -- Tiempo transcurrido en minutos: {:?}, ", ((ahora - antes)/1000)/60);
}



pub fn abre_fichero(fichero: &str) -> bool{
    let mut arr_parti_txt: Vec<Vec<String>> = Vec::new();
    let mut lineas_partida: Vec<String> = Vec::new();
    let mut arr_todas_partidas: Vec<PartiT> = Vec::new();
    let mut contador: i16 = 0;
    
    let partida_t = PartiT::init();
    
    let f = match fs::File::open(fichero) {
        Ok(fich) => fich,
        Err(_) => return false,
    };
    
    let fich = BufReader::new(&f);
    
    for linea in fich.lines() {
        let line = linea.unwrap();
        
        if line.starts_with("[Event ") && lineas_partida.len() > 0{
            arr_parti_txt.push(lineas_partida);
            // vaciamos el vector para la sig. partida
            lineas_partida = Vec::new();    //.clear();
        }
        
        lineas_partida.push(line.to_string());
    }
    // la ultima partida del fichero que se ha quedado colgada en el for ...
    arr_parti_txt.push(lineas_partida);
    
    let ncpus = num_cpus::get();
    let pool = ThreadPool::new(ncpus);
    for elem in arr_parti_txt{
        let (tx, rx) = mpsc::channel();
        let mut parti_t = partida_t.clone();
        //let mut validada = partida_t.clone();
        pool.execute(move || {
            let mut l1 = elem.clone();
            let validada = ahorma_partida(&mut parti_t, &mut l1);
            tx.send(validada).unwrap();
        });
        let received = rx.recv().unwrap();
        // si la longitud es =2 => erronea
        contador += 1;
        if contador%100 == 0 {
            println!("239 - {} --> {} -- {}", contador, received.cabecera[4], received.cabecera[5]);
        }
        arr_todas_partidas.push(received);
        if arr_todas_partidas.len() >= 500 {
            println!("243 --> Grabando...");
            graba_partidas(&mut arr_todas_partidas);
            arr_todas_partidas = Vec::new();
            println!("246 --> Validando...");
        }
    }
    
    pool.join();
    if arr_todas_partidas.len() > 0 {
        println!("252 --> Grabando...");
        graba_partidas(&mut arr_todas_partidas);
    }
    
    true
}



fn ahorma_partida(partida: &mut PartiT, parti: &mut Vec<String>) -> PartiT{
    let mut campos_cabeza: HashMap<String, String> = HashMap::new();
    let mut partida_s: String = "".to_string();
    let re = Regex::new(r"\[(\w*)\s*(.+)]").unwrap();
    let mut es_cabecera: bool = true;
    for i in 0..parti.len(){
        // entre la cabecera y el cuerpo de la partida hay una linea en blanco
        if parti[i].len() == 0 {
            es_cabecera = false;
        }
        
        if parti[i].starts_with('[') && es_cabecera {
            /*
            NOTA:
            Hay programas como Chessbase que usan algunos comentarios raros.
            por ejemplo Anand-Nakamura, Tal Memorial, 2013, ronda 6
            En la linea 157;
            ---
            Qxe8 21. h4 g4 22. Nh2 {Abergel,T (2517)-Kosten,A (2497)/Nimes/2009/} g3 $36 {
            [%csl Gb7,Gg7]}) 10. Be3 O-O 11. h3 Bxf3 12. Qxf3 f5 (12... Bxd4 13. Rad1 c5
            ---
            la linea del comentario empieza con [
            Las lineas de cabecera suelen tener una longitud menor y la linea de partida 
            es siempre mayor que 60, excepto la ultima linea.
            */
            let coincide = re.captures(&parti[i]).unwrap();
            // [1] es la clave [2] es el valor, [o] es el valor orig.
            //Hay que quitar las dobles comilla al campo 2
            let mut comillas: String = coincide[2].to_string();
            comillas.remove(0);
            comillas.pop();
            campos_cabeza.insert(coincide[1].to_string(), comillas.to_string());
        }
        else{
            // hay que poner un " " para reemplazar el "\n"
            partida_s.push_str(" ");
            partida_s.push_str(&parti[i]);
        }
    }
    
    // vamos a sacar, por orden, los campos de la cabecera ordenados
    // para el registro en la tabla de la BD
    let mut campo = "Event".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.replace("'", " ").to_string();
            //token = token.replace("'", " ");
            partida.cambia_cabecera(y);
        }
    }
    campo = "Site".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.replace("'", " ").to_string();
            partida.cambia_cabecera(y);
        }
    }
    campo = "Date".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.to_string();
            partida.cambia_cabecera(y);
        }
    }
    campo = "Round".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.to_string();
            partida.cambia_cabecera(y);
        }
    }
    campo = "White".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.replace("'", " ").to_string();
            partida.cambia_cabecera(y);
        }
    }
    campo = "Black".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.replace("'", " ").to_string();
            partida.cambia_cabecera(y);
        }
    }
    campo = "Result".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.to_string();
            partida.cambia_cabecera(y);
        }
    }
    campo = "ECO".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.to_string();
            partida.cambia_cabecera(y);
        }
    }
    else {
        partida.cambia_cabecera("???".to_string());
    }
    
    campo = "WhiteElo".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.to_string();
            partida.cambia_cabecera(y);
        }
    }
    else {
        partida.cambia_cabecera("0".to_string());
    }
    
    campo = "BlackElo".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.to_string();
            partida.cambia_cabecera(y);
        }
    }
    else {
        partida.cambia_cabecera("0".to_string());
    }
    
    campo = "FEN".to_string();
    if campos_cabeza.contains_key(&campo) {
        if let Some(x) = campos_cabeza.get_mut(&campo) {
            let y: String = x.to_string();
            partida.cambia_cabecera(y);
        }
    }
    else {
        partida.cambia_cabecera("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    }
    
    // ahora añadimos el pgn
    partida.cambia_pgn(partida_s.trim().to_string());
    // hacemos un array de movims.
    let mut parti_clon = partida.clone();
    ahorma_jugadas(&mut parti_clon);
    
    let retorno = parti_clon.clone();
    retorno
}



fn ahorma_jugadas (mut partida: &mut PartiT){ 
    let mut token = partida.pgn.clone();
    let mut pos: usize;
    let mut comentario: bool = false;
    
    // para quitar los num. de jugada
    let re = Regex::new(r"^\s*(\d+\.+\s*)?").unwrap();
    
    // quitamos / añadimos espacios y comillas simples
    token = token.replace("(", "( ");
    token = token.replace(")", " )");
    token = token.replace("'", " ");
    token = token.replace("\"", " ");
    token = token.trim().to_string();
    
    while token.len() > 0 {
        token = re.replace(token.as_str(), "").to_string();
        pos = 0;
        
        if token.starts_with("{"){
            if token.find("}") == None {
                
            }
            else{
                pos = token.find("}").unwrap() + 1;
                comentario = true;
            }
        }
        else{
            if token.find(" ") == None {
            
            }
            else{
                pos = token.find(" ").unwrap();
            }
        }
        
        if pos > 0 {
            partida.movims.push(token[..pos].to_string());
            token = token[pos..].to_string().replace("\\", " ");
            if comentario{
                pos = 0;
                comentario = false;
            }
            else{
                pos -= 1;
            }
        }
        else{
            partida.movims.push(token);
            token = "".to_string();
        }
    }
    // borramos los posibles elementos vacios (espacios)
    partida.movims.retain(|x| x != " ");
    
    valida_partida(&mut partida);
}



fn valida_partida(mut partida: &mut PartiT) {
    let mut partida_valida: bool = true;
    partida_valida = linea_principal(&mut partida, 0 as usize);   // la linea principal será la 0 (cero)
    if partida_valida {
        partida_valida = linea_variantes(&mut partida);
    }
    if !partida_valida {
        let primero = MoveT::crea_jugada("Var0Mv0".to_string(), "".to_string(), "".to_string(),
                    "0".to_string(),"".to_string(),
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
                    "".to_string(),"".to_string(),"0".to_string()); 
        let ultimo = MoveT::crea_jugada("Var0Mv1".to_string(), partida.cabecera[6].to_string(),
                    partida.cabecera[6].to_string(),"1".to_string(),"".to_string(),"".to_string(),
                    "".to_string(),"".to_string(),"0".to_string());
        let mut partida_vacia: Vec<MoveT> = Vec::new();
        partida_vacia.push(primero);
        partida_vacia.push(ultimo);
        partida.arr_partida = Vec::new();
        partida.arr_partida.push(partida_vacia);
    }
    partida.partida_json = serde_json::to_string(&partida.arr_partida).unwrap();
}



fn linea_principal(partida: &mut PartiT, numvar: usize) -> bool{
    let nivel: usize = numvar.clone();     //numero de variante
    let FEN: String;     // = String::new();
    // para controlar cuando se inicia y cierra una variante
    // se inicia con '(' y cuando length = 0 (despues de ')')
    let mut num_parentesis: i16 = 0;
    // para tener controlada las fen de cada jugada 
    let mut arr_fen: Vec<String> = Vec::new();
    // para el numero de jugadas 
    let mut num_jugada:i16 = 0;
    // creamos un array temporal para las variantes
    let mut arr_var: Vec<MoveT> = Vec::new();
    
    // obtenemos la fen de la partida
    let temp = &partida.cabecera[10];
    
    // quitamos las comillas iniciales y finales
    FEN = temp.to_string();
    
    // iniciamos el tablero vacio
    let mut board = ajedrez::Tablero::init();
    // inicializamos con la FEN
    let fen_valida = ajedrez::set_fen(&FEN, &mut board);
    if !fen_valida {return false;}
    // creamos el indice 0 vacío solo con la fen inicial 
    // la san será una cadena vacía asi:
    // (indActual, SAN, UCI, num_jugada, turno, fen, nag, comentarios, array_subvariantes, profundidad)
    // indice actual = "Var"+num_variante+"Mv"+num_jugada
    let ind_actual = format!("Var{}Mv{}", nivel, num_jugada);
    let jug = MoveT::crea_jugada(ind_actual.to_string(), "".to_string(), "".to_string(), num_jugada.to_string(), "".to_string(), 
                    FEN.to_string(), "".to_string(), "".to_string(), nivel.to_string());
    
    arr_var.push(jug);
    partida.arr_partida.push(arr_var);
    let mut v_fen = FEN.clone();
    arr_fen.push(v_fen);
    arr_var = Vec::new();
    
    for index in 0..partida.movims.len(){
        let ult_elem: usize = partida.arr_partida[nivel].len() - 1;
        let long_arr_parti = partida.arr_partida.len();
        let mut long_arr_var = arr_var.len();
        let mut jug: MoveT;
        let mut turno_jugado: String;
        let mut num_jug_var: i16 = num_jugada;
        
        v_fen = FEN.clone();
        let iter = v_fen.split_whitespace();
        let fen_dividida = iter.collect::<Vec<&str>>();
        
        if fen_dividida[1] == "w" {
            turno_jugado = "b".to_string();
        }
        else {
            turno_jugado = "w".to_string();
        }
            
        // comprobamos si hay un nag o comentario
        // esta comprobacion es solo para la lin.principal
        if num_parentesis == 0 {
            if partida.movims[index].starts_with("$") {
                partida.arr_partida[nivel][ult_elem].nag = partida.movims[index].to_string();
                continue;
            }
            if partida.movims[index].starts_with("{"){
                partida.arr_partida[nivel][ult_elem].comen = partida.movims[index].to_string();
                continue;
            }
        }
        
        // empiezan las posibles variantes
        if partida.movims[index] == "(" {
            // para controlar apertura y cierre de variante
            num_parentesis += 1;
        }
        
        if num_parentesis > 0 {  
            // estamos en una variante y no lo validamos ahora
            // se hará en la funcion linea_variante()
            if num_parentesis == 1 && partida.movims[index] == "(" {
                // se inicia una variante nueva
                // (indActual, SAN, UCI, num_jugada, turno, fen, nag, comentarios, array_subvariantes, profundidad)
                jug = MoveT::crea_jugada(format!("Var{}Mv{}", long_arr_parti, num_jug_var), 
                                        partida.movims[index].to_string(),
                                        partida.movims[index].to_string(),
                                        num_jug_var.to_string(),
                                        turno_jugado.to_string(),
                                        arr_fen[arr_fen.len()-2].to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        (nivel + 1).to_string(),
                                        );
                num_jug_var += 1;
            }
            else{
                // solo ponemos la SAN. En la func. linea_variantes se arreglará y validará
                // cada jugada de las variantes y subvariantes
                num_jug_var += 1;
                // (indActual, SAN, UCI, num_jugada, turno, fen, nag, comentarios, array_subvariantes, profundidad)
                jug = MoveT::crea_jugada(format!("Var{}Mv{}", long_arr_parti, num_jug_var), 
                                        partida.movims[index].to_string(),
                                        partida.movims[index].to_string(),
                                        num_jug_var.to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        (nivel + 1).to_string(),
                                        );
            }
            // obtenemos el primer caracter para comprobar si es un movim. o no.
            let ch = partida.movims[index].chars().next().unwrap();
            
            if ch.to_string() != "$" && ch.to_string() != "{" {
                arr_var.push(jug);
            }
            else if partida.movims[index].starts_with("$") {
                let l = arr_var.len() - 1;
                arr_var[l].nag = partida.movims[index].to_string();
            }
            else if partida.movims[index].starts_with("{") {
                let l = arr_var.len() - 1;
                arr_var[l].comen = partida.movims[index].to_string();
            }
            
            if partida.movims[index].starts_with(")") {
                num_parentesis -= 1;
                if num_parentesis == 0{
                    partida.arr_partida.push(arr_var);
                    arr_var = Vec::new();
                    // ahora creo el enlace a este nuevo array /variante
                    // la ultima variante coincide con la logitu del arr_partida
                    let l1 = partida.arr_partida[nivel].len()-1;
                    let l2 = partida.arr_partida.len()-1;
                    partida.arr_partida[nivel][l1].sub_var.push(l2);
                }
            }
        }
        
        else {
            // comprobamos si la jugada es valida
            let _resultado = ajedrez::mueve_san(&mut board, &partida.movims[index]);
            if _resultado.0 != "None"{
                let v_fen_1 = ajedrez::get_fen(&mut board);
                let _san = _resultado.0.to_string();
                let _uci = _resultado.1.to_string();
                let _turn_jugado = _resultado.2.to_string();    //String::new();
                
                //si la partida comienza con el turno de las negras
                if num_jugada == 0 && _turn_jugado == "b" {
                    num_jugada += 1;
                }
                num_jugada += 1;
                // (indActual, SAN, UCI, num_jugada, turno, fen, nag, comentarios, array_subvariantes, profundidad)
                jug = MoveT::crea_jugada(format!("Var{}Mv{}", long_arr_var, num_jugada), 
                                        _san.to_string(),
                                        _uci.to_string(),
                                        num_jugada.to_string(),
                                        _turn_jugado.to_string(),
                                        v_fen_1.to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        nivel.to_string(),
                                        );
                arr_fen.push(v_fen_1);
                partida.arr_partida[nivel].push(jug);
            }
            else {
                // se mutila, ante cualquier error, la partida.
                // da siempre el error fundamentalmente al leer el resultado
                // ahora añadimos el resultado
                num_jugada += 1;
                jug = MoveT::crea_jugada(format!("Var{}Mv{}", long_arr_var, num_jugada), 
                                        partida.cabecera[6].to_string(),
                                        partida.cabecera[6].to_string(),
                                        num_jugada.to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        "".to_string(),
                                        nivel.to_string(),
                                        );
                partida.arr_partida[nivel].push(jug);
                //por tanto no retornamos false
                //return false;
            }
        }
    }
    true
}


fn linea_variantes(parti: &mut PartiT) -> bool {
    let mut parentesis: i16;     // = 0;
    // creamos un array temporal para las variantes
    let mut arr_var: Vec<MoveT>;    // = Vec::new();
    
    // para tener controlada las fen de cada jugada 
    let mut arr_fen: Vec<String>;   // = Vec::new();
    let mut idx_par: usize = 1;
    let mut long_parti = parti.arr_partida.len();
    // el bucle empieza =1, porque el idx_par = 0 es la linea principal
    while idx_par < long_parti {
        parentesis = 0;
        arr_var = Vec::new();
        let mut num_jugada: i16 = parti.arr_partida[idx_par][0].num_jug.parse::<i16>().unwrap();
        let mut profundidad: i16 = 0;
        // posiciones inicio y final de cada subvariante 
        // para su borrante en la variante padre
        let mut pos_ini_subvar: usize = 0;
        let mut pos_fin_subvar: usize;  // = 0;
        
        // iniciamos el tablero vacio
        let mut board = ajedrez::Tablero::init();
        arr_fen = Vec::new();
        let FEN = parti.arr_partida[idx_par][0].fen.to_string();
        let v_fen = FEN.clone();
        let fen_valida = ajedrez::set_fen(&v_fen, &mut board);
        if !fen_valida {
            // la partida se mutila
            return false;
        }
        //el primer elem es '(' buscaremos a ver si hay alguna subvariante '('
        let mut idx_var: usize = 1;
        let long_var = parti.arr_partida[idx_par].len() - 1;
        
        while idx_var <= long_var {
            // aqui va el control de parentesis
            if parti.arr_partida[idx_par][idx_var].san == "(" {
                // para controlar apertura y cierre de variante
                parentesis += 1;
                // en caso de mas de un '(' 
                if parentesis == 1 {
                    //calculamos la profundidad de la sub-sub-variante
                    profundidad = parti.arr_partida[idx_par][0].profundidad.parse::<i16>().unwrap() + 1;
                    
                    // creamos el primer elemento de la nueva variante
                    // necesito hacer una copia profunda
                    let l_fen: usize = arr_fen.len()-2;
                    let copia = parti.arr_partida[idx_par][idx_var].clone();
                    arr_var.push(copia);
                    arr_var[0].profundidad = profundidad.to_string();
                    let fen_temp = &arr_fen[l_fen];
                    arr_var[0].fen = fen_temp.to_string();
                    // regeheramos el idx_jug es la longitud de arr_partida, 
                    // ya que este será el indice cuando añadamos.
                    let jug_tmp = arr_var[0].num_jug.to_string();
                    arr_var[0].idx_jug = format!("Var{}Mv{}", parti.arr_partida.len(), jug_tmp);
                    
                    // tomamos la posición de inicio y final para su borrado
                    // cuando parentesis = 0
                    pos_ini_subvar = idx_var;
                    pos_fin_subvar = idx_var;
                    // queda abierta la subvariante
                    idx_var += 1;
                    continue;
                }
            }
            
            if parentesis > 0 {
                pos_fin_subvar = idx_var;
                let copia = parti.arr_partida[idx_par][idx_var].clone();
                arr_var.push(copia);
                // modificamos la profundidad de cada subvariante
                let l_arr_var: usize = arr_var.len()-1;
                arr_var[l_arr_var].profundidad = profundidad.to_string();
                
                // el cierre de una sub-variante
                if parti.arr_partida[idx_par][idx_var].san == ")" {
                    parentesis -= 1;
                    if parentesis == 0 {
                        parti.arr_partida.push(arr_var);
                        arr_var = Vec::new();
                        // borramos los elementos traspasados a la nueva subvariante
                        for i in (pos_ini_subvar..pos_fin_subvar + 1).rev(){
                            parti.arr_partida[idx_par].remove(i);
                            idx_var -= 1;   //la longitud de la variante está disminuyendo
                        }
                        
                        // hacemos el enlace en la variante padre
                        // [0] ...
                        // ...
                        // [n+1] = long_parti
                        parti.arr_partida[idx_par][pos_ini_subvar - 1].sub_var.push(long_parti);
                        
                        // aumentamos el tamaño del bucle en 1
                        // ya la longitud de parti.arr_partida ha aumentado en 1
                        long_parti += 1;
                    }
                }
                
            }
            
            else {
                if parti.arr_partida[idx_par][idx_var].san == ")" {
                    num_jugada += 1;
                    parti.arr_partida[idx_par][idx_var].idx_jug = format!("Var{}Mv{}", idx_par, num_jugada);
                    parti.arr_partida[idx_par][idx_var].num_jug = num_jugada.to_string();
                    break;
                }
                
                let _resultado = ajedrez::mueve_san(&mut board, &parti.arr_partida[idx_par][idx_var].san);
                if _resultado.0 != "None"{
                    let v_fen_1 = ajedrez::get_fen(&mut board);
                    let _san = _resultado.0.to_string();
                    let _uci = _resultado.1.to_string();
                    let mut _turn_jugado = String::new();
                    
                    if _resultado.2 == "w".to_string() {
                        _turn_jugado = "b".to_string();
                    }
                    else {
                        _turn_jugado = "w".to_string();
                    }
                    parti.arr_partida[idx_par][idx_var].uci = _uci.to_string();
                    parti.arr_partida[idx_par][idx_var].fen = v_fen_1.to_string();
                    parti.arr_partida[idx_par][idx_var].turno = _turn_jugado;
                    num_jugada += 1;
                    parti.arr_partida[idx_par][idx_var].num_jug = num_jugada.to_string();
                    parti.arr_partida[idx_par][idx_var].idx_jug = format!("Var{}Mv{}", idx_par, num_jugada);
                    arr_fen.push(v_fen_1);
                }
                else {
                    //println!("782 --> Truncando partida :  {:?}", parti.cabecera);
                    //panic!("\n563 Error jugada -- {} - {}", parti.arr_partida[idx_par][idx_var].san, ajedrez::get_fen(&mut board));
                    return false;
                }
            }
            
            idx_var += 1;
        }
        idx_par += 1;
    }
    true
}



fn graba_partidas(lista: &mut Vec<PartiT>) {
    let mut conn = Connection::open("sqlite_database/bd_ajedrez.db").expect("Fallo en la conexión a la BD");
    /*
    let insert_sql = "INSERT INTO tb_partidas (
                            evento,
                            sitio,
                            fecha, 
                            ronda, 
                            blancas,
                            negras,
                            resultado, 
                            ECO, 
                            elo_blancas, 
                            elo_negras,
                            FEN,
                            pgn,
                            html
                        ) 
                        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)";
                
    let tran = conn.transaction().unwrap();
    let stmt =  tran.prepare(insert_sql).expect("Prepare ha fallado");
    */
    let tran = conn.transaction().unwrap();
    for parti in lista {
        let pa = RegistroBD {
            num_partida : 0,
            evento : parti.cabecera[0].to_string(),
            r_site : parti.cabecera[1].to_string(),
            r_date : parti.cabecera[2].to_string(),
            r_round : parti.cabecera[3].to_string(),
            r_white : parti.cabecera[4].to_string(),
            r_black : parti.cabecera[5].to_string(),
            r_result : parti.cabecera[6].to_string(),
            r_eco : parti.cabecera[7].to_string(),
            r_whiteelo : parti.cabecera[8].to_string(),
            r_blackelo : parti.cabecera[9].to_string(),
            r_fen : parti.cabecera[10].to_string(),
            r_pgn : parti.pgn.to_string(),
            r_validada : parti.partida_json.to_string(),
            r_indexado : 0,
            r_borrado : 0,
        };
        
        //let rn = stmt.execute(
        
        let _rn = tran.execute("INSERT INTO tb_partidas (
                            evento,
                            sitio,
                            fecha, 
                            ronda, 
                            blancas,
                            negras,
                            resultado, 
                            ECO, 
                            elo_blancas, 
                            elo_negras,
                            FEN,
                            pgn,
                            html,
                            indexado,
                            borrado
                        ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14, ?15)",
            &[&pa.evento as &ToSql, &pa.r_site, &pa.r_date, &pa.r_round, &pa.r_white,
                &pa.r_black, &pa.r_result, &pa.r_eco, &pa.r_whiteelo, &pa.r_blackelo,
                &pa.r_fen, &pa.r_pgn, &pa.r_validada, &pa.r_indexado, &pa.r_borrado]
        ).unwrap();
    }
    tran.commit().unwrap();
}



fn exporta_partidas(){
    struct Cx {
        c01 : i32,
        c02 : String,
        c03 : String,
        c04 : String,
        c05 : String,
        c06 : String,
        c07 : String,
        c08 : String,
        c09 : String,
        c10 : String,
        c11 : String,
        c12 : String,
        partida: String,
    }
    println!("Exportando partidas a formato JSON ... ");
    
    let conn = Connection::open("sqlite_database/bd_ajedrez.db").unwrap();
    let mut stmt = conn.
        prepare("SELECT * FROM tb_partidas")
        .unwrap();
        
    let partida_iter = stmt
        .query_map(NO_PARAMS, |row| Cx {
            c01: row.get(0),
            c02 : row.get(1),
            c03 : row.get(2),
            c04 : row.get(3),
            c05 : row.get(4),
            c06 : row.get(5),
            c07 : row.get(6),
            c08 : row.get(7),
            c09 : row.get(8),
            c10 : row.get(9),
            c11 : row.get(10),
            c12 : row.get(11),
            partida: row.get(13),
        })
        .unwrap();
    
    let mut hilos = vec![];
    for partida in partida_iter {
        let _par: Cx = partida.unwrap();
        let cab = vec![_par.c01.to_string(), _par.c02, _par.c03, _par.c04, _par.c05, _par.c06, _par.c07, _par.c08, _par.c09, _par.c10, _par.c11, _par.c12];
        let exporta = ExportaJSON {
            cabecera :cab,
            partida : serde_json::from_str(&_par.partida).unwrap(),
        };
        let partida_json = serde_json::to_string(&exporta).unwrap();
        let (tx, rx) = mpsc::channel();
        let hilo_actual = thread::spawn(move|| {
            tx.send(graba_fichero_json(partida_json)).unwrap();
        });
        rx.recv().unwrap();
        
        hilos.push(hilo_actual);
    }
    
    for i in hilos {
        let _ = i.join();
    }
}



fn graba_fichero_json(partida: String) {
    let path = "html/partidas_json.js";
    let f = File::open(&path);
    
    if f.is_err(){
        let mut f_hash = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        if let Err(e) = writeln!(f_hash, "{}", partida) {
            println!("\n990 No se puede escribir en el fichero: {}", e);
        }
        
    }
    else {
        let mut f_hash = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap();
        if let Err(e) = writeln!(f_hash, "{}", partida) {
            println!("\n1001 No se puede escribir en el fichero: {}", e);
        }
    }
}
