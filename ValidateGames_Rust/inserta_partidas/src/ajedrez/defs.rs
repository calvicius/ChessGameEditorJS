/*
Contiene constantes usadas en varias clases
*/

// Constantes para el tipo de movimiento
pub const ORDINARY_MOVE: i16 = 0;
pub const SHORT_CASTLE: i16 = 1;
pub const LONG_CASTLE: i16 = 2;
pub const EN_PASSANT: i16 = 3;
pub const PROMOTION_QUEEN: i16 = 4;
pub const PROMOTION_ROOK: i16 = 5;
pub const PROMOTION_BISHOP: i16 = 6;
pub const PROMOTION_KNIGHT: i16 = 7;


// Bando a mover
pub const WHITE_TO_MOVE: i16 = 1;
pub const BLACK_TO_MOVE: i16 = -1;


//Constantes para las piezas
pub const W_KING: i16 = 1;
pub const W_QUEEN: i16 = 2;
pub const W_ROOK: i16 = 3;
pub const W_BISHOP: i16 = 4;
pub const W_KNIGHT: i16 = 5;
pub const W_PAWN: i16 = 6;

pub const B_KING: i16 = -1;
pub const B_QUEEN: i16 = -2;
pub const B_ROOK: i16 = -3;
pub const B_BISHOP: i16 = -4;
pub const B_KNIGHT: i16 = -5;
pub const B_PAWN: i16 = -6;

pub const EMPTY_SQUARE: i16 = 0;


//Constantes para la disponibilidades de enroques
pub const CASTLE_NONE: i16 = 0;
pub const CASTLE_SHORT: i16 = 1;
pub const CASTLE_LONG: i16 = 2;
pub const CASTLE_BOTH: i16 = 3;


// Deltas de las piezas
pub const BISHOP_DELTA: [i16;8] = [-15, -17, 15, 17, 0, 0, 0, 0];
pub const ROOK_DELTA: [i16;8] = [-1, -16, 1, 16, 0, 0, 0, 0];
pub const QUEEN_DELTA: [i16;8] = [-15, -17, 15, 17, -1, -16, 1, 16];
pub const KING_DELTA: [i16;8] = [-15, -17, 15, 17, -1, -16, 1, 16];
pub const KNIGHT_DELTA: [i16;8] = [18, 33, 31, 14, -31, -33, -18, -14];
pub const PAWN_DELTA: [i16;8] = [16, 32, 17, 15, 0, 0, 0, 0];


// Constantes, array de Ataque, array de deltas
pub const ATTACK_NONE: i16 = 0;     // Deltas que ninguna pieza puede mover.
pub const ATTACK_KQR: i16 = 1;      // Una casilla arriba abajo izquierda y derecha
pub const ATTACK_QR: i16 = 2;       // Más de una casilla arriba abajo izquierda y derecha
pub const ATTACK_KQB_WP: i16 = 3;    // Una casilla en diagonal hacia arriba
pub const ATTACK_KQB_BP: i16 = 4;    // Una casilla en diagonal hacia abajo
pub const ATTACK_QB: i16 = 5;       // Mas de una casilla en diagonal
pub const ATTACK_N: i16 = 6;        // Movimientos de caballo

// Formula: casilla_atacada - casilla_atacante + 128 = piezas capaces de atacar

pub const ATTACK_ARRAY: [i16; 257] =[
    0,0,0,0,0,0,0,0,0,5,0,0,0,0,0,0,2,0,0,0,
    0,0,0,5,0,0,5,0,0,0,0,0,2,0,0,0,0,0,5,0,
    0,0,0,5,0,0,0,0,2,0,0,0,0,5,0,0,0,0,0,0,
    5,0,0,0,2,0,0,0,5,0,0,0,0,0,0,0,0,5,0,0,
    2,0,0,5,0,0,0,0,0,0,0,0,0,0,5,6,2,6,5,0,
    0,0,0,0,0,0,0,0,0,0,6,4,1,4,6,0,0,0,0,0,
    0,2,2,2,2,2,2,1,0,1,2,2,2,2,2,2,0,0,0,0,
    0,0,6,3,1,3,6,0,0,0,0,0,0,0,0,0,0,0,5,6,
    2,6,5,0,0,0,0,0,0,0,0,0,0,5,0,0,2,0,0,5,
    0,0,0,0,0,0,0,0,5,0,0,0,2,0,0,0,5,0,0,0,
    0,0,0,5,0,0,0,0,2,0,0,0,0,5,0,0,0,0,5,0,
    0,0,0,0,2,0,0,0,0,0,5,0,0,5,0,0,0,0,0,0,
    2,0,0,0,0,0,0,5,0,0,0,0,0,0,0,0,0
    ];
    
// Igual que el array de ataque, pero da el delta/desplazamiento necesario para llegar a la casilla
    
pub const DELTA_ARRAY: [i16; 257] =[
   0,   0,   0,   0,   0,   0,   0,   0,   0, -17,   0,   0,   0,   0,   0,   0, -16,   0,   0,   0,
   0,   0,   0, -15,   0,   0, -17,   0,   0,   0,   0,   0, -16,   0,   0,   0,   0,   0, -15,   0,
   0,   0,   0, -17,   0,   0,   0,   0, -16,   0,   0,   0,   0, -15,   0,   0,   0,   0,   0,   0,
 -17,   0,   0,   0, -16,   0,   0,   0, -15,   0,   0,   0,   0,   0,   0,   0,   0, -17,   0,   0,
 -16,   0,   0, -15,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, -17, -33, -16, -31, -15,   0,
   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, -18, -17, -16, -15, -14,   0,   0,   0,   0,   0,
   0,  -1,  -1,  -1,  -1,  -1,  -1,  -1,   0,   1,   1,   1,   1,   1,   1,   1,   0,   0,   0,   0,
   0,   0,  14,  15,  16,  17,  18,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,  15,  31,
  16,  33,  17,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,  15,   0,   0,  16,   0,   0,  17,
   0,   0,   0,   0,   0,   0,   0,   0,  15,   0,   0,   0,  16,   0,   0,   0,  17,   0,   0,   0,
   0,   0,   0,  15,   0,   0,   0,   0,  16,   0,   0,   0,   0,  17,   0,   0,   0,   0,  15,   0,
   0,   0,   0,   0,  16,   0,   0,   0,   0,   0,  17,   0,   0,  15,   0,   0,   0,   0,   0,   0,
  16,   0,   0,   0,   0,   0,   0,  17,   0,   0,   0,   0,   0,   0,   0,   0,   0
  ];


// nombre de las casillas
pub const SQUARE_NAMES: [&'static str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"
    ];
  

//pub const PIECE_SYMBOLS: [&'static str; 7] = ["", "P", "N", "B", "R", "Q", "K"];

//pub const FILE_NAMES: [&'static str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

//pub const RANK_NAMES: [&'static str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];

#[derive(Copy, Clone)]
pub struct CasillaVal {
    pub casilla: &'static str,
    pub indice : i16
}

pub const CASILLAS_VALOR: [CasillaVal; 64] = [
    // fila 8
    CasillaVal{casilla:"a8", indice:112},
    CasillaVal{casilla:"b8", indice:113},
    CasillaVal{casilla:"c8", indice:114},
    CasillaVal{casilla:"d8", indice:115},
    CasillaVal{casilla:"e8", indice:116},
    CasillaVal{casilla:"f8", indice:117},
    CasillaVal{casilla:"g8", indice:118},
    CasillaVal{casilla:"h8", indice:119},
    // fila 7
    CasillaVal{casilla:"a7", indice: 96},
    CasillaVal{casilla:"b7", indice: 97},
    CasillaVal{casilla:"c7", indice: 98},
    CasillaVal{casilla:"d7", indice: 99},
    CasillaVal{casilla:"e7", indice:100},
    CasillaVal{casilla:"f7", indice:101},
    CasillaVal{casilla:"g7", indice:102},
    CasillaVal{casilla:"h7", indice:103},
    // fila 6
    CasillaVal{casilla:"a6", indice:80},
    CasillaVal{casilla:"b6", indice:81},
    CasillaVal{casilla:"c6", indice:82},
    CasillaVal{casilla:"d6", indice:83},
    CasillaVal{casilla:"e6", indice:84},
    CasillaVal{casilla:"f6", indice:85},
    CasillaVal{casilla:"g6", indice:86},
    CasillaVal{casilla:"h6", indice:87},
    // fila 5
    CasillaVal{casilla:"a5", indice:64},
    CasillaVal{casilla:"b5", indice:65},
    CasillaVal{casilla:"c5", indice:66},
    CasillaVal{casilla:"d5", indice:67},
    CasillaVal{casilla:"e5", indice:68},
    CasillaVal{casilla:"f5", indice:69},
    CasillaVal{casilla:"g5", indice:70},
    CasillaVal{casilla:"h5", indice:71},
    // fila 4
    CasillaVal{casilla:"a4", indice:48},
    CasillaVal{casilla:"b4", indice:49},
    CasillaVal{casilla:"c4", indice:50},
    CasillaVal{casilla:"d4", indice:51},
    CasillaVal{casilla:"e4", indice:52},
    CasillaVal{casilla:"f4", indice:53},
    CasillaVal{casilla:"g4", indice:54},
    CasillaVal{casilla:"h4", indice:55},
    // fila 3
    CasillaVal{casilla:"a3", indice:32},
    CasillaVal{casilla:"b3", indice:33},
    CasillaVal{casilla:"c3", indice:34},
    CasillaVal{casilla:"d3", indice:35},
    CasillaVal{casilla:"e3", indice:36},
    CasillaVal{casilla:"f3", indice:37},
    CasillaVal{casilla:"g3", indice:38},
    CasillaVal{casilla:"h3", indice:39},
    // fila 2
    CasillaVal{casilla:"a2", indice:16},
    CasillaVal{casilla:"b2", indice:17},
    CasillaVal{casilla:"c2", indice:18},
    CasillaVal{casilla:"d2", indice:19},
    CasillaVal{casilla:"e2", indice:20},
    CasillaVal{casilla:"f2", indice:21},
    CasillaVal{casilla:"g2", indice:22},
    CasillaVal{casilla:"h2", indice:23},
    // fila 1
    CasillaVal{casilla:"a1", indice:0},
    CasillaVal{casilla:"b1", indice:1},
    CasillaVal{casilla:"c1", indice:2},
    CasillaVal{casilla:"d1", indice:3},
    CasillaVal{casilla:"e1", indice:4},
    CasillaVal{casilla:"f1", indice:5},
    CasillaVal{casilla:"g1", indice:6},
    CasillaVal{casilla:"h1", indice:7}
    ];

    
#[derive(Copy, Clone)]
pub struct Cols {
    pub col: &'static str,
    pub inds : [i16; 8]
}

pub const COLUMNAS: [Cols; 8] = [
    Cols{col:"a", inds:[0,16,32,48,64,80,96,112]},
    Cols{col:"b", inds:[1,17,33,49,65,81,97,113]},
    Cols{col:"c", inds:[2,18,34,50,66,82,98,114]},
    Cols{col:"d", inds:[3,19,35,51,67,83,99,115]},
    Cols{col:"e", inds:[4,20,36,52,68,84,100,116]},
    Cols{col:"f", inds:[5,21,37,53,69,85,101,117]},
    Cols{col:"g", inds:[6,22,38,54,70,86,102,118]},
    Cols{col:"h", inds:[7,23,39,55,71,87,103,119]}
];


#[derive(Copy, Clone)]
pub struct Filas {
    pub fila: i16,
    pub inds : [i16; 8]
}

pub const FILAS: [Filas; 8] = [
    Filas{fila:1, inds:[0,1,2,3,4,5,6,7]},
    Filas{fila:2, inds:[16,17,18,19,20,21,22,23]},
    Filas{fila:3, inds:[32,33,34,35,36,37,38,39]},
    Filas{fila:4, inds:[48,49,50,51,52,53,54,55]},
    Filas{fila:5, inds:[64,65,66,67,68,69,70,71]},
    Filas{fila:6, inds:[80,81,82,83,84,85,86,87]},
    Filas{fila:7, inds:[96,97,98,99,100,101,102,103]},
    Filas{fila:8, inds:[112,113,114,115,116,117,118,119]}
];
