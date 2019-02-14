'use strict'


/*
Variablex globales
*/

//variables internas
var gl_arrPartida = [];
var gl_arrCabecera = [];
var gl_tablero_inicial = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
var gl_partida_dict;    // es la partida_json en formato {cabecera: [...], partida: [...]}
var gl_jugadaActual;    // será la estructura de function struct_jugada
var gl_foco;            //para que la SAN actual quede resaltada

var game = new Chess();


// Primero el tablero
// **************************************************
// Aqui empieza la parte del tablero, exclusivamente
// **************************************************
// variables del tablero
var board;
var statusEl = $('#status');
var fenEl = $('#fen');

var removeGreySquares = function() {
  $('#board .square-55d63').css('background', '');
};

var greySquare = function(square) {
  var squareEl = $('#board .square-' + square);
  
  var background = '#a9a9a9';
  if (squareEl.hasClass('black-3c85d') === true) {
    background = '#696969';
  }

  squareEl.css('background', background);
};

var onDragStart = function(source, piece) {
  // do not pick up pieces if the game is over
  // or if it's not that side's turn
  if (game.game_over() === true ||
      (game.turn() === 'w' && piece.search(/^b/) !== -1) ||
      (game.turn() === 'b' && piece.search(/^w/) !== -1)) {
    return false;
  }
};

var onDrop = function(source, target) {
  removeGreySquares();

  // see if the move is legal
  var move = game.move({
    from: source,
    to: target,
    //promotion: 'q' 
    promotion: document.getElementById("promote").value
  });

  // illegal move
  if (move === null) return 'snapback';
  // la jugada es valida. Construimos el movim.
  move.promotion = (move.promotion === undefined) ? "" : move.promotion;
  move.fen = game.fen();
  //console.log(43, move, move.from + move.to + move.promotion);
  jugada_tablero(move);
};

var onMouseoverSquare = function(square, piece) {
  // get list of possible moves for this square
  var moves = game.moves({
    square: square,
    verbose: true
  });

  // exit if there are no moves available for this square
  if (moves.length === 0) return;

  // highlight the square they moused over
  greySquare(square);

  // highlight the possible squares for this piece
  for (var i = 0; i < moves.length; i++) {
    greySquare(moves[i].to);
  }
};

var onMouseoutSquare = function(square, piece) {
  removeGreySquares();
};

var onSnapEnd = function() {
  board.position(game.fen());
};

var updateStatus = function() {
  var status = '';

  var moveColor = 'Blancas';
  if (game.turn() === 'b') {
    moveColor = 'Negras';
  }

  // checkmate?
  if (game.in_checkmate() === true) {
    status = 'Partida, ' + moveColor + ' es jaque mate.';
  }

  // draw?
  else if (game.in_draw() === true) {
    status = 'Fin de Partida, posición de empate';
  }

  // game still on
  else {
    status = moveColor + ' mueven.' ; // Se ha efectuado la jugada núm. ' + parseInt(numJugadaActiva);

    // check?
    if (game.in_check() === true) {
      status += '. ' + moveColor + ' están en jaque';
    }
  }

  statusEl.html(status);
  fenEl.html(game.fen());
  //pgnEl.html(game.pgn());
};

var cfg = {
  pieceTheme: 'chessboardjs/img/chesspieces/wikipedia/{piece}.png',
  draggable: true,
  showNotation: false,
  showErrors: true,
  position: gl_tablero_inicial, 
  onDragStart: onDragStart,
  onDrop: onDrop,
  onMouseoutSquare: onMouseoutSquare,
  onMouseoverSquare: onMouseoverSquare,
  onSnapEnd: onSnapEnd
};
board = ChessBoard('board', cfg);


// ahora la parte interna
//inicia(partida_json);
//***************************************************
// se inician las funciones
//***************************************************
function struct_jugada (idx_jug, san, uci, num_jug, turno, fen, nag, comen, sub_var, profundidad) {
    this.idx_jug = idx_jug;
    this.san = san;                 //: String,
    this.uci = uci;                 //: String,
    this.num_jug = num_jug;             //: String,
    this.turno = turno;               //: String, 
    this.fen = fen;                 //: String,
    this.nag = nag;                 //: String,
    this.comen = comen;               //: String,
    this.sub_var = sub_var;             //: array de variantes;las eventuales variantes que cuelgen de este movim.
    this.profundidad = profundidad;         //: String, esto es a la hora de imprimir (margenes, saltos de linea, etc...)
}


function inicia (partida_json){
    if (partida_json.length == 0) {
        partida_json = '{"cabecera":["0","?","?","????.??.??","?","?","?","*","???","0","0","rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"],"partida":[[{"idx_jug":"Var0Mv0","san":"","uci":"","num_jug":"0","turno":"","fen":"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1","nag":"","comen":"","sub_var":[],"profundidad":"0"},{"idx_jug":"Var0Mv1","san":"*","uci":"*","num_jug":"1","turno":"","fen":"","nag":"","comen":"","sub_var":[],"profundidad":"0"}]]}';
    }
    // es la partida_json en formato {cabecera: [...], partida: [...]}
    var partida_dict = JSON.parse(partida_json);
    var arrCabecera = partida_dict.cabecera;
    // inicializamos algunas variables globales
    gl_arrPartida = partida_dict.partida;
    gl_jugadaActual = gl_arrPartida[0][0];
    gl_foco = gl_jugadaActual.idx_jug;
    
    var v_fen = gl_arrPartida[0][0].fen;
    presenta_cabecera(arrCabecera);
    board.position(v_fen);
    game = new Chess(v_fen);
    // presentamos la partida
    var html = creaHtml();
    $("#div_html_partida").html(html);
    updateStatus();
}


function presenta_cabecera(arrCabecera) {
    $('#evento').html(arrCabecera[1]);
    $('#sitio').html(arrCabecera[2]);
    $('#fecha').html(arrCabecera[3]);
    $('#ronda').html(arrCabecera[4]);
    $('#blancas').html(arrCabecera[5]);
    $('#negras').html(arrCabecera[6]);
    $('#resultado').html(arrCabecera[7]);
    $('#eco').html(arrCabecera[8]);
    $('#elo_blancas').html(arrCabecera[9]);
    $('#elo_negras').html(arrCabecera[10]);
    $('#cab_fen').html(arrCabecera[11]);
}

function jugada_tablero(movim) {
    /*
    if (typeof jugadaActual === undefined) {
        gl_jugadaActual = gl_arrPartida[0][0];
    }
    */
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
    var num_elem =  gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);                //    parseInt(arr_idx[1]);
    /*
    // los elementos de gl_jugadaActual
    var idx_jug = String(gl_jugadaActual.idx_jug);
    var san = gl_jugadaActual.san;
    var uci = gl_jugadaActual.uci;
    var num_jug = gl_jugadaActual.num_jug;
    var turno = gl_jugadaActual.turno;
    var fen = gl_jugadaActual.fen;
    var nag = gl_jugadaActual.nag;
    var comen = gl_jugadaActual.comen;
    var sub_var = gl_jugadaActual.sub_var;
    var profundidad = gl_jugadaActual.profundidad;
    */
    
    if ((movim.from + movim.to + movim.promotion) == gl_arrPartida[num_var][num_elem+1].uci) {
        gl_jugadaActual = gl_arrPartida[num_var][num_elem+1];
        // los elementos de gl_jugadaActual
        var idx_jug = String(gl_jugadaActual.idx_jug);
        var san = gl_jugadaActual.san;
        var uci = gl_jugadaActual.uci;
        var num_jug = gl_jugadaActual.num_jug;
        var turno = gl_jugadaActual.turno;
        var fen = gl_jugadaActual.fen;
        var nag = gl_jugadaActual.nag;
        var comen = gl_jugadaActual.comen;
        var sub_var = gl_jugadaActual.sub_var;
        var profundidad = gl_jugadaActual.profundidad;
        muestra_jugada(idx_jug, san, uci, num_jug, turno, fen, nag, comen, sub_var, profundidad);
    }
    else {
        // se añade una jugada al final de la variante
        if (gl_arrPartida[num_var][num_elem+1].san == "0-1" || 
                gl_arrPartida[num_var][num_elem+1].san == "1-0" || 
                gl_arrPartida[num_var][num_elem+1].san == "1/2-1/2" || 
                gl_arrPartida[num_var][num_elem+1].san == "*" || 
                gl_arrPartida[num_var][num_elem+1].san == ")") {
            anade_jugada_final(movim);
        }
        else {
            anade_jugada_mitad(movim);
        }
    }
}


function anade_jugada_final(movim){
    //obtenemos el elem siguiente a la jugada actual
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
    var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
    var jugada_siguiente = gl_arrPartida[num_var][num_elem+1];
    
    // ahora creamos la estructura de la nueva jugada
    gl_jugadaActual = {
        idx_jug: jugada_siguiente.idx_jug, 
        san: movim.san, 
        uci: movim.from + movim.to + movim.promotion,   //uci, 
        num_jug: jugada_siguiente.num_jug, 
        turno: movim.color,     //turno, 
        fen: movim.fen, 
        nag: "", 
        comen: "", 
        sub_var: [], 
        profundidad: jugada_siguiente.profundidad
    };
    //eliminamos el ultimo elemento de la variante
    var elem_borrado = gl_arrPartida[num_var].pop();
    //añadimos la nueva jugada
    gl_arrPartida[num_var].push(gl_jugadaActual);
    // modificamos algunos datos del elemento borrado 
    var ult_num_jug = parseInt(jugada_siguiente.num_jug) + 1;
    jugada_siguiente.idx_jug = "Var" + String(num_var) + "Mv" + String(ult_num_jug);
    jugada_siguiente.num_jug = String(ult_num_jug);
    gl_arrPartida[num_var].push(jugada_siguiente);
    // rehacemos el texto de la partida
    muestra_jugada(gl_jugadaActual.idx_jug, gl_jugadaActual.san, gl_jugadaActual.uci,
                gl_jugadaActual.num_jug, gl_jugadaActual.turno, gl_jugadaActual.fen,
                gl_jugadaActual.nag, gl_jugadaActual.comen, gl_jugadaActual.sub_var,
                gl_jugadaActual, gl_jugadaActual.profundidad);
}
    

function anade_jugada_mitad(movim) {
    //obtenemos el elem siguiente a la jugada actual
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
    var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
    var jugada_siguiente = gl_arrPartida[num_var][num_elem+1];
    //antes de continuar miramos si la jug. siguiente tiene
    // entre sus subvariantes una coincidente con el movim.
    if (jugada_siguiente.sub_var.length > 0) {
        // tenemos subvariantes...
        var coincide = false;
        var num_arr = 0;
        for (var i=0; i < jugada_siguiente.sub_var.length; i++) {
            num_arr = jugada_siguiente.sub_var[i];
            if (gl_arrPartida[num_arr][1].uci == movim.from+movim.to+movim.promotion){
                coincide = true;
                break;
            }
        }
        if (coincide) {
            // el elemento coincidente sera gl_arrPartida[num_arr][1]
            var el = gl_arrPartida[num_arr][1];
            muestra_jugada(el.idx_jug, el.san, el.uci, el.num_jug, el.turno, el.fen, el.nag, el.comen, el.sub_var, el.profundidad);
            return;
        }
    }
    //creamos un array temporal para la nueva variante
    var arr_temp = [];
    var primer_elem = {
        idx_jug: "Var" + String(gl_arrPartida.length) + "Mv" + gl_jugadaActual.num_jug,
        san: "(", 
        uci: "(", 
        num_jug: gl_jugadaActual.num_jug, 
        turno: jugada_siguiente.turno, 
        fen: gl_jugadaActual.fen, 
        nag: "", 
        comen: "", 
        sub_var: [], 
        profundidad: String(parseInt(gl_jugadaActual.profundidad) + 1)
    };
    var c01 = "Var" + String(gl_arrPartida.length) + "Mv" + String(parseInt(gl_jugadaActual.num_jug) + 1);
    var c02 = movim.san;
    var c03 = movim.from + movim.to + movim.promotion;
    var c04 = String(parseInt(gl_jugadaActual.num_jug) + 1);
    var c05 = movim.color;
    var c06 = movim.fen;
    var c07 = "";
    var c08 = "";
    var c09 = [];
    var c10 = String(parseInt(gl_jugadaActual.profundidad) + 1);
    var jugada = {
        idx_jug: c01,
        san: c02, 
        uci: c03,
        num_jug: c04, 
        turno: c05,
        fen: c06, 
        nag: c07, 
        comen: c08, 
        sub_var: c09, 
        profundidad: c10
    };
    var ult_elem = {
        idx_jug: "Var" + String(gl_arrPartida.length) + "Mv" + String(parseInt(gl_jugadaActual.num_jug) + 2),
        san: ")", 
        uci: ")",   //uci, 
        num_jug: String(parseInt(gl_jugadaActual.num_jug) + 2), 
        turno: "",     //turno, 
        fen: "", 
        nag: "", 
        comen: "", 
        sub_var: [], 
        profundidad: String(parseInt(gl_jugadaActual.profundidad) + 1)
    };
    
    arr_temp.push(primer_elem);
    arr_temp.push(jugada),
    arr_temp.push(ult_elem);
    // añadimos al array de la partida
    gl_arrPartida.push(arr_temp);
    // añadimos el num. de array a la jugada padre
    jugada_siguiente = gl_arrPartida[num_var][num_elem+1].sub_var.push(gl_arrPartida.length - 1);
    
    // mostramos la nueva jugada
    muestra_jugada(c01, c02, c03, c04, c05, c06, c07, c08, c09,c10);
    
    // para que se vea el elemento automaticamente cuando la variante es larga
    var elmnt = document.getElementById(gl_arrPartida[gl_arrPartida.length -1][1].idx_jug);
    elmnt.scrollIntoView(false); // false para que se vea abajo
}


function muestra_jugada(idx_jug, san, uci, num_jug, turno, fen, nag, comen, sub_var, profundidad) {
    gl_jugadaActual = {
        idx_jug: idx_jug, 
        san: san, 
        uci: uci, 
        num_jug: num_jug, 
        turno: turno, 
        fen: fen, 
        nag: nag, 
        comen: comen, 
        sub_var: sub_var, 
        profundidad: profundidad
    };
    
    // posición de la barra de scroll
    var pos_scroll = div_html_partida.scrollTop;
    
    gl_foco = idx_jug;
    
    board.position(fen);
    fenEl.html(fen);
    game.load(fen);
    
    // ahora el textarea de los comentarios
    //para modificar el comentario primero sacamos si hay algun comentario anterior, pero antes lo limpiamos
    $('textarea#areaComenta').val("");	//limpio el textarea del comentario anterior
    $('textarea#areaComenta').val(gl_jugadaActual.comen);
    // grabamos el comentario
    $("#grabaComen").on('click', function(){
        var v_com = jQuery("textarea#areaComenta").val();
        //limpio el textarea del posible comentario anterior
        $('textarea#areaComenta').val("");	
        //tenemos que evitar las comillas dobles y simples para que no interfiera con el href onclick
        //las comillas parten la frase var = 'pepe' la transformariamos en var = 'pe'pe' lo que da error
        v_com = v_com.replace("'", " ");
        v_com = v_com.replace('"', ' ');
        gl_jugadaActual.comen = v_com;
        
        // vamos a grabar en el elemento correspondiente el comentario
        var indice = (gl_jugadaActual.idx_jug).split("Var");
        var arr_idx = indice[1].split("Mv");
        var num_var = parseInt(arr_idx[0]);
        
        var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
        gl_arrPartida[num_var][num_elem].comen = v_com;
        $('textarea#areaComenta').val(v_com);
        var html = creaHtml();
        $("#div_html_partida").html(html);
        document.getElementById("div_html_partida").scrollTop = pos_scroll;
        //document.getElementById(gl_foco).style.backgroundColor = "yellow";
    });
    
    var html = creaHtml();
    $("#div_html_partida").html(html);
    document.getElementById("div_html_partida").scrollTop = pos_scroll;
    document.getElementById(gl_foco).style.backgroundColor = "yellow";
    updateStatus();
}


/***********************************************************
* parte relativa a los NAG formato Informator
***********************************************************/

function insertaNAG() {
    // anotacion de nag
    if (gl_foco != "Var0Mv0") {
        var indice = (gl_jugadaActual.idx_jug).split("Var");
        var arr_idx = indice[1].split("Mv");
        var num_var = parseInt(arr_idx[0]);
        
        if (document.getElementById("informator").value == "$0") {
            gl_jugadaActual.nag = "";
            var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
            gl_arrPartida[num_var][num_elem].nag = "";
        }
        else {
            gl_jugadaActual.nag = gl_jugadaActual.nag + document.getElementById("informator").value;
            var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
            gl_arrPartida[num_var][num_elem].nag = gl_arrPartida[num_var][num_elem].nag + document.getElementById("informator").value;
        }
        var html = creaHtml();
        $("#div_html_partida").html(html);
        document.getElementById(gl_foco).style.backgroundColor = "yellow";
    }
    document.getElementById("informator").selectedIndex = 0;
}


/*
Creacion de las NAG para la impresion del html
Idea de http://pgn4web.casaschi.net
*/

function leeNAG(cadena){
    var ii = 0;
    var retorno = '';
    
    var Ns = '<span class="NAGs">';
    var Nl = '<span class="NAGl">';
    var Ne = '</span>';
    
    var NAG = [];
    NAG[0] = '';
    NAG[1] = '!';  // 'good move';
    NAG[2] = '?';  // 'bad move';
    NAG[3] = '!!'; // 'very good move';
    NAG[4] = '??'; // 'very bad move';
    NAG[5] = '!?'; // 'speculative move';
    NAG[6] = '?!'; // 'questionable move';
    NAG[7] = NAG[8] = Ns + '&#236;' + Ne; // 'forced move';
    NAG[9] = '??'; // 'worst move';
    NAG[10] = NAG[11] = NAG[12] = Ns + '&#61;' + Ne; // 'drawish position';
    NAG[13] = Ns + '&#213;' + Ne; // 'unclear position';
    NAG[14] = Ns + '&#162;' + Ne; // 'White has a slight advantage';
    NAG[15] = Ns + '&#163;' + Ne; // 'Black has a slight advantage';
    NAG[16] = Ns + '&#165;' + Ne; // 'White has a moderate advantage';
    NAG[17] = Ns + '&#164;' + Ne; // 'Black has a moderate advantage';
    NAG[18] = NAG[20] = Ns + '&#43;&#187;' + Ne; // 'White has a decisive advantage';
    NAG[19] = NAG[21] = Ns + '&#187;&#43;' + Ne; // 'Black has a decisive advantage';
    NAG[22] = NAG[23] = Ns + '&#194;' + Ne; // 'zugzwang';
    NAG[24] = NAG[25] = NAG[26] = NAG[27] = NAG[28] = NAG[29] = Ns + '&#193;' + Ne; // 'space advantage';
    NAG[30] = NAG[31] = NAG[32] = NAG[33] = NAG[34] = NAG[35] = Ns + '&#182;' + Ne; // 'time (development) advantage';
    NAG[36] = NAG[37] = NAG[38] = NAG[39] = Ns + '&#238;' + Ne; // 'initiative';
    NAG[40] = NAG[41] = Ns + '&#239;' + Ne; // 'attack';
    NAG[42] = NAG[43] = ''; // 'insufficient compensation for material deficit';
    NAG[44] = NAG[45] = NAG[46] = NAG[47] = Ns + '&#167;' + Ne; // 'sufficient compensation for material deficit';
    NAG[48] = NAG[49] = NAG[50] = NAG[51] = NAG[52] = NAG[53] = Ns + '&#191;' + Ne; // 'center control advantage';
    for (ii = 54; ii <= 129; ii++) { NAG[ii] = ''; }
    NAG[130] = NAG[131] = NAG[132] = NAG[133] = NAG[134] = NAG[135] = Ns + '&#124;' + Ne; // 'counterplay';
    NAG[136] = NAG[137] = NAG[138] = NAG[139] = Ns + '&#176;' + Ne; // 'time control pressure';

    NAG[140] = Nl + '&#197;' + Ne; // 'with the idea';
    NAG[141] = ''; // 'aimed against';
    NAG[142] = Nl + '&#196;' + Ne; // 'better is';
    NAG[143] = ''; // 'worse is';
    NAG[144] = Nl + '&#61;' + Ne; // 'equivalent is';
    NAG[145] = 'RR'; // 'editorial comment';
    NAG[146] = 'N'; // 'novelty';
    NAG[147] = NAG[244] = Nl + '&#94;' + Ne; // 'weak point';
    NAG[148] = NAG[245] = Nl + '&#207;' + Ne; // 'endgame';
    NAG[149] = NAG[239] = Nl + '&nbsp;&nbsp;&#732;&nbsp;' + Ne; // 'file';
    NAG[150] = NAG[240] = Nl + '&#92;' + Ne; // 'diagonal';
    NAG[151] = NAG[152] = NAG[246] = Nl + '&#210;' + Ne; // 'bishop pair';
    NAG[153] = NAG[247] = Nl + '&#211;' + Ne; // 'opposite bishops';
    NAG[154] = NAG[248] = Nl + '&#212;' + Ne; // 'same bishops';
    NAG[155] = NAG[156] = NAG[193] = NAG[249] = Nl + '&#217;' + Ne; // 'connected pawns';
    NAG[157] = NAG[158] = NAG[192] = NAG[250] = Nl + '&#219;' + Ne; // 'isolated pawns';
    NAG[159] = NAG[160] = NAG[191] = NAG[251] = Nl + '&#218;' + Ne; // 'doubled pawns';
    NAG[161] = NAG[162] = NAG[252] = Nl + '&#8249;' + Ne; // 'passed pawn';
    NAG[163] = NAG[164] = NAG[253] = Nl + '&#8250;' + Ne; // 'pawn majority';
    for (ii = 165; ii <= 189; ii++) { NAG[ii] = ''; }
    NAG[190] = Nl + '&#223;' + Ne; // 'etc';
    NAG[194] = ''; // 'hanging pawns';
    NAG[195] = ''; // 'backward pawns';
    for (ii = 196; ii <= 200; ii++) { NAG[ii] = ''; }
    NAG[201] = NAG[220] = NAG[221] = ''; // 'diagram';
    for (ii = 202; ii <= 219; ii++) { NAG[ii] = ''; }
    for (ii = 222; ii <= 237; ii++) { NAG[ii] = ''; }
    NAG[238] = Nl + '&#193;' + Ne; // 'space advantage';
    NAG[241] = Nl + '&#191;' + Ne; // 'center';
    NAG[242] = Nl + '&#125;' + Ne; // 'kingside';
    NAG[243] = Nl + '&#123;' + Ne; // 'queenside';
    NAG[254] = Nl + '&#8216;' + Ne; // 'with';
    NAG[255] = Nl + '&#95;' + Ne; // 'without';
    
    //las NAG pueden ser del tipo $2$123
    var numNAG = []; // el numero de cada NAG
    numNAG = cadena.split('$');
    
    if (numNAG.length > 1) {
        for (ii = 1; ii < numNAG.length; ++ii){
            retorno += NAG[numNAG[ii]];
        }
        return retorno;
    } else {
        return (Ns + '' + Ne);
    }
}

/* FIN de las NAG */
/* ====================================== */



/******************************************************** 
* Creacion de la partida en formato html
* Trabajaremos sobre un div con id='div_html_partida'
* Los margenes de los div internos seran de de 5 en 5px
********************************************************/
function creaVarHtml(numVar){
    var idx = parseInt(numVar);
    var retorno = '';
    var arrJugadas = [];
    
    var vFen = '';
    var arrFen = [];
    var numJugada = '';
    var turno = '';
    
    var profun = 0;
    /* para controlar la numeracion de la primera jugada de cada variante */
    var anadidaJug = false;
    /* si no hay un enlace a una subvariante */
    /*
    var vVar = gl_arrPartida.filter(function(e) { return e.san === 'VAR'; }).length;   // =0 si no lo encuentra
    
    if (vVar === 0){
        var vProf = gl_arrPartida[numVar][0].profundidad;
        arrJugadas.push("<div id = 'divVar_" + vProf.toString() + "'>");
    }
    */
    /* tomamos el elem[0] que es igual a '(' */
    arrJugadas.push('<span class="movim">' + gl_arrPartida[idx][0].san + '</span>');
    
    for (var i = 1; i < gl_arrPartida[idx].length; ++i){
        //if (gl_arrPartida[idx][i].san != 'VAR'){
        vFen = gl_arrPartida[idx][i].fen;
        arrFen = vFen.split(' ');
        numJugada = arrFen[5];
        turno = arrFen[1];
        
        if (!anadidaJug){
            if (turno === 'w'){ //han jugado las negras
                arrJugadas.push('<span>' + parseInt(numJugada - 1).toString() + '. ...' + '</span>');
            } else if (turno === 'b'){ //han jugado las blancas
                arrJugadas.push('<span>' + numJugada + '.' + '</span>');
            }
            anadidaJug = true;
        }
        else if (arrJugadas.length > 1){
            if (turno === 'b'){ //han jugado las blancas
                arrJugadas.push('<span>' + numJugada + '.' + '</span>');
            }
            if (turno === 'w' && arrJugadas[arrJugadas.length - 1] === '</div>'){
                arrJugadas.push('<span>' + parseInt(numJugada - 1).toString() + '. ...' + '</span>');
            }
        }
        
        var idx_jug = gl_arrPartida[idx][i].idx_jug;
        var san = gl_arrPartida[idx][i].san;
        var uci = gl_arrPartida[idx][i].uci;
        var num_jug = gl_arrPartida[idx][i].num_jug;
        var turno = gl_arrPartida[idx][i].turno;
        var fen = gl_arrPartida[idx][i].fen;
        var nag = gl_arrPartida[idx][i].nag;
        var comen = gl_arrPartida[idx][i].comen;
        var sub_var = gl_arrPartida[idx][i].sub_var;
        var profundidad = gl_arrPartida[idx][i].profundidad;
        
        if (san == "(" || san == ")") {
            arrJugadas.push('<span>' + san + '</span>');
        }
        else {
            arrJugadas.push("<span class='movim' id='" + idx_jug + "' tabindex='-1'>" +
                        "<a href='#' onclick= \"muestra_jugada(" + "'" + idx_jug + "', '" +
                        san + "', '" + uci + "', '" + num_jug + "', '" + turno + "', '" + fen + "', '" + nag + "', '" + comen +
                        "', '" + sub_var + "', '" + profundidad +
                        "'" + ")\">" + san + '</a></span>');
        }
        arrJugadas.push(leeNAG(gl_arrPartida[idx][i].nag));
        gl_arrPartida[idx][i].comen = gl_arrPartida[idx][i].comen.replace(/{/g, '');
        gl_arrPartida[idx][i].comen = gl_arrPartida[idx][i].comen.replace(/}/g, '');
        arrJugadas.push('<span>' + gl_arrPartida[idx][i].comen + '</span>');
        
        // variable para recuperar el num variante una vez
        //retorne de la subvariante
        var indViejo = idx;
        for (var j = 0; j < gl_arrPartida[idx][i].sub_var.length; ++j){
            var hijo = gl_arrPartida[idx][i].sub_var[j];
            var arrSubVar = creaVarHtml(gl_arrPartida[idx][i].sub_var[j]);
            // tomamos el elem[0] para obtener la profundidad 
            profun = gl_arrPartida[parseInt(hijo)][0].profundidad;
            arrJugadas.push("<div id = 'divVar_" + profun.toString() + "'>");
            arrJugadas.push.apply(arrJugadas, arrSubVar);
            arrJugadas.push('</div>');
        }
        idx = indViejo;
    }
    /*
    if (vVar == 0){
        arrJugadas.push("</div>");
    }
    */
    return arrJugadas;
}



function creaHtml(){
    var textoHtml = '';
    var arrJugadas = [];
    
    var vFen = '';
    var arrFen = [];
    var numJugada = '';
    var turno = '';
    
    for (var i = 1; i < gl_arrPartida[0].length; ++i){
        
        vFen = gl_arrPartida[0][i].fen;
        arrFen = vFen.split(' ');
        numJugada = Math.ceil(gl_arrPartida[0][i].num_jug / 2);       //arrFen[5];
        
        if (!arrJugadas.length){
            if (gl_arrPartida[0][i].turno === 'b'){ //han jugado las negras
                arrJugadas.push('<span>' + parseInt(numJugada).toString() + '. ...' + '</span>')
            } else if (gl_arrPartida[0][i].turno === 'w'){ //han jugado las blancas
                arrJugadas.push('<span>' + numJugada + '.' + '</span>')
            }
        }
        else {
            if (gl_arrPartida[0][i].turno === 'w'){ //han jugado las blancas
                arrJugadas.push('<span>' + numJugada + '.' + '</span>');
            }
            if (gl_arrPartida[0][i].turno === 'b' && arrJugadas[arrJugadas.length - 1] === '</div>'){
                arrJugadas.push('<span>' + parseInt(numJugada).toString() + '. ...' + '</span>');
            }
        }
        
        var idx_jug = gl_arrPartida[0][i].idx_jug;
        var san = gl_arrPartida[0][i].san;
        var uci = gl_arrPartida[0][i].uci;
        var num_jug = gl_arrPartida[0][i].num_jug;
        var turno = gl_arrPartida[0][i].turno;
        var fen = gl_arrPartida[0][i].fen;
        var nag = gl_arrPartida[0][i].nag;
        var comen = gl_arrPartida[0][i].comen;
        var sub_var = gl_arrPartida[0][i].sub_var;
        var profundidad = gl_arrPartida[0][i].profundidad;
        
        if (san == "0-1" || san == "1-0" || san == "1/2-1/2" || san == "*") {
            arrJugadas.push('<span>' + san + '</span>');
        }
        else {
            arrJugadas.push("<span class='movim' id='" + idx_jug + "' tabindex='-1'>" +
                        "<a href='#' onclick= \"muestra_jugada(" + "'" + idx_jug + "', '" +
                        san + "', '" + uci + "', '" + num_jug + "', '" + turno + "', '" + fen + "', '" + nag + "', '" + comen +
                        "', '" + sub_var + "', '" + profundidad +
                        "'" + ")\">" + gl_arrPartida[0][i].san + '</a></span>');
        }
        
        arrJugadas.push(leeNAG(gl_arrPartida[0][i].nag));
        gl_arrPartida[0][i].comen = gl_arrPartida[0][i].comen.replace(/{/g, '');
        gl_arrPartida[0][i].comen = gl_arrPartida[0][i].comen.replace(/}/g, '');
        arrJugadas.push('<span>' + gl_arrPartida[0][i].comen + '</span>');
        
        for (var j = 0; j < gl_arrPartida[0][i].sub_var.length; ++j){
            var arrVar = creaVarHtml(gl_arrPartida[0][i].sub_var[j]);
            // tomamos el elem[0] para obtener la profundidad 
            var profun = gl_arrPartida[gl_arrPartida[0][i].sub_var[j]][0].profundidad;
            arrJugadas.push("<div id = 'divVar_" + profun.toString() + "'>");
            arrJugadas.push.apply(arrJugadas, arrVar);
            arrJugadas.push('</div>');
        }
    }
    textoHtml = arrJugadas.join(' ');
    return textoHtml;
}    
// ***************** FIN de la creacion la partida en html ******************    


/****************************************************
* creación de la partida en formato PGN
****************************************************/

function creaPgn(){
    var textoPgn = '';
    var arrJugadas = [];
    
    var vFen = '';
    var arrFen = [];
    var numJugada = '';
    var turno = '';
    
    for (var i = 1; i < gl_arrPartida[0].length; ++i){
        vFen = gl_arrPartida[0][i].fen;
        arrFen = vFen.split(' ');
        numJugada = Math.ceil(gl_arrPartida[0][i].num_jug / 2);
        
        if (!arrJugadas.length){
            if (gl_arrPartida[0][i].turno === 'b'){ //han jugado las negras
                arrJugadas.push(parseInt(numJugada).toString() + '. ...')
            } else if (gl_arrPartida[0][i].turno === 'w'){ //han jugado las blancas
                arrJugadas.push(numJugada + '.')
            }
        }
        else {
            if (gl_arrPartida[0][i].turno === 'w'){ //han jugado las blancas
                arrJugadas.push(numJugada + '.');
            }
            if (gl_arrPartida[0][i].turno === 'b' && arrJugadas[arrJugadas.length-1] == ')'){
                arrJugadas.push(parseInt(numJugada).toString() + '. ...');
            }
        }
        
        arrJugadas.push(gl_arrPartida[0][i].san);
        if (gl_arrPartida[0][i].comen.nag > 0) {
            arrJugadas.push(gl_arrPartida[0][i].nag);
        }
        if (gl_arrPartida[0][i].comen.length > 0) {
            arrJugadas.push('{' + gl_arrPartida[0][i].comen + '}');
        }
        
        for (var j = 0; j < gl_arrPartida[0][i].sub_var.length; ++j){
            var arrVar = creaVarPgn(gl_arrPartida[0][i].sub_var[j]);
            // tomamos el elem[0] para obtener la profundidad 
            //var profun = gl_arrPartida[gl_arrPartida[0][i].sub_var[j]][0].profundidad;
            //arrJugadas.push("<div id = 'divVar_" + profun.toString() + "'>");
            arrJugadas.push.apply(arrJugadas, arrVar);
            //arrJugadas.push('</div>');
        }
    }
    textoPgn = arrJugadas.join(' ');
    textoPgn = textoPgn.replace('  ', ' ');
    //textoPgn = textoPgn.replace(/(\s/g, '(');
    //textoPgn = textoPgn.replace(/\s)/g, ')');
    //console.log(textoPgn);
    return textoPgn;
}


function creaVarPgn(numVar){
    var idx = parseInt(numVar);
    var retorno = '';
    var arrJugadas = [];
    
    var vFen = '';
    var arrFen = [];
    var numJugada = '';
    var turno = '';
    
    var profun = 0;
    /* para controlar la numeracion de la primera jugada de cada variante */
    var anadidaJug = false;
    /* tomamos el elem[0] que es igual a '(' */
    arrJugadas.push(gl_arrPartida[idx][0].san);

    for (var i = 1; i < gl_arrPartida[idx].length; ++i){
        //if (gl_arrPartida[idx][i].san != 'VAR'){
        vFen = gl_arrPartida[idx][i].fen;
        arrFen = vFen.split(' ');
        numJugada = arrFen[5];
        turno = arrFen[1];
        
        if (!anadidaJug){
            if (turno === 'w'){ //han jugado las negras
                arrJugadas.push(parseInt(numJugada - 1).toString() + '. ...');
            } else if (turno === 'b'){ //han jugado las blancas
                arrJugadas.push(numJugada + '.');
            }
            anadidaJug = true;
        }
        else if (arrJugadas.length > 1){
            if (turno === 'b'){ //han jugado las blancas
                arrJugadas.push(numJugada + '.');
            }
            if (turno === 'w' && (arrJugadas[arrJugadas.length-1] == ')' || arrJugadas[arrJugadas.length-1] == '(')){
                arrJugadas.push(parseInt(numJugada - 1).toString() + '. ...');
            }
        }
        
        arrJugadas.push(gl_arrPartida[idx][i].san);
        if (gl_arrPartida[idx][i].nag.length > 0) {
            arrJugadas.push(gl_arrPartida[idx][i].nag);
        }
        if (gl_arrPartida[idx][i].comen.length > 0) {
            arrJugadas.push('{' + gl_arrPartida[idx][i].comen +'}');
        }
        // variable para recuperar el num variante una vez
        //retorne de la subvariante
        var indViejo = idx;
        for (var j = 0; j < gl_arrPartida[idx][i].sub_var.length; ++j){
            var hijo = gl_arrPartida[idx][i].sub_var[j];
            var arrSubVar = creaVarPgn(gl_arrPartida[idx][i].sub_var[j]);
            // tomamos el elem[0] para obtener la profundidad 
            //profun = gl_arrPartida[parseInt(hijo)][0].profundidad;
            arrJugadas.push.apply(arrJugadas, arrSubVar);
        }
        idx = indViejo;
    }
    
    return arrJugadas;
}


function set_crea_pgn() {
    var pgn_txt = creaPgn();
    $("#areaComenta").val(pgn_txt);
    //$("#areaComenta").html(pgn_txt);
}

//************** FIN crea partida en formato PGN *****



/*****************************************************
* Apartado de los botones del tablero
*****************************************************/

$("#btnVoltea").on('click', board.flip);


$("#btnFinal").on('click', function(){
    // comprobamos que estamos en algun sitio
    if (gl_jugadaActual.idx_jug == "Var0Mv0") {
        return;
    }
    // obtenermos el num_array y elemento de la jugada actual
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
	var ultima = gl_arrPartida[num_var][gl_arrPartida[num_var].length - 2];
    
	// para que se vea el elemento automaticamente cuando la variante es larga
    var elmnt = document.getElementById(ultima.idx_jug);
    elmnt.scrollIntoView(false); 

    muestra_jugada(ultima.idx_jug, ultima.san, ultima.uci, ultima.num_jug, ultima.turno, ultima.fen, 
                    ultima.nag, ultima.comen, ultima.sub_var, ultima.profundidad);
});


$("#btnInicio").on('click', function(){
    // comprobamos que estamos en algun sitio
    if (gl_jugadaActual.idx_jug == "Var0Mv0") {
        return;
    }
    // obtenermos el num_array y elemento de la jugada actual
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
	var primera = gl_arrPartida[num_var][1];
    
	// para que se vea el elemento automaticamente cuando la variante es larga
    var elmnt = document.getElementById(primera.idx_jug);
    elmnt.scrollIntoView(true); // se muestra arriba

    muestra_jugada(primera.idx_jug, primera.san, primera.uci, primera.num_jug, primera.turno, primera.fen, 
                    primera.nag, primera.comen, primera.sub_var, primera.profundidad);
});


$("#btnAnterior").on('click', function(){
    // comprobamos que estamos en algun sitio
    if (gl_jugadaActual.idx_jug == "Var0Mv0") {
        return;
    }
    // obtenermos el num_array y elemento de la jugada actual
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
    var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
    // la jugada anterior
    var ant = gl_arrPartida[num_var][num_elem - 1];
    // para que se vea el elemento automaticamente cuando la variante es larga
    var elmnt = document.getElementById(ant.idx_jug);
    elmnt.scrollIntoView(true); // se muestra arriba
    muestra_jugada(ant.idx_jug, ant.san, ant.uci, ant.num_jug, ant.turno, ant.fen, 
                    ant.nag, ant.comen, ant.sub_var, ant.profundidad);
});



$("#btnSiguiente").on('click', function(){
    // comprobamos que estamos en algun sitio
    if (gl_jugadaActual.idx_jug == "Var0Mv0") {
        return;
    }
	// obtenermos el num_array y elemento de la jugada actual
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
    var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
    // la jugada anterior
    var sig = gl_arrPartida[num_var][num_elem + 1];
    // para que se vea el elemento automaticamente cuando la variante es larga
    var elmnt = document.getElementById(sig.idx_jug);
    elmnt.scrollIntoView(false); // se muestra abajo
    muestra_jugada(sig.idx_jug, sig.san, sig.uci, sig.num_jug, sig.turno, sig.fen, 
                    sig.nag, sig.comen, sig.sub_var, sig.profundidad);
});


$("#btnBorraJug").on('click', function(){
    // comprobamos que estamos en algun sitio
    if (gl_jugadaActual.idx_jug == "Var0Mv0") {
        return;
    }
    var movi = {};
    var para_borrar = [];
    // obtenemos el num_array y elemento de la jugada actual
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
    var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
    var longitud = gl_arrPartida[num_var].length;
    //eliminamos el ultimo elemento de la variante
    var ultimo = gl_arrPartida[num_var].pop();
    //borramos hasta el final
    gl_arrPartida[num_var].length = num_elem + 1;
    //var elem_borrados = gl_arrPartida[num_var].splice(num_elem+1, longitud - 3);
    var num_jug_final = parseInt(gl_arrPartida[num_var][num_elem].num_jug) + 1;
    var idx_final = "Var" + String(num_var) + "Mv" + String(num_jug_final);
    ultimo.idx_jug = idx_final;
    ultimo.num_jug = String(num_jug_final);
    gl_arrPartida[num_var].push(ultimo);
    
    muestra_jugada(gl_jugadaActual.idx_jug, gl_jugadaActual.san, gl_jugadaActual.uci, 
                    gl_jugadaActual.num_jug, gl_jugadaActual.turno, gl_jugadaActual.fen, 
                    gl_jugadaActual.nag, gl_jugadaActual.comen, gl_jugadaActual.sub_var, gl_jugadaActual.profundidad);
});


$("#btnBorraVar").on('click', function() {
    // comprobamos que estamos en algun sitio
    if (gl_jugadaActual.idx_jug == "Var0Mv0") {
        return;
    }
    // obtenermos el num_array y elemento de la jugada actual
    var indice = (gl_jugadaActual.idx_jug).split("Var");
    var arr_idx = indice[1].split("Mv");
    var num_var = parseInt(arr_idx[0]);
    var num_elem = gl_arrPartida[num_var].findIndex(x => x.idx_jug==gl_jugadaActual.idx_jug);
    
    // la variante principal no se permite borrarla
    if (num_var > 0) {
        //buscamos el nodo padre
        for (var i=0; i < gl_arrPartida.length; i++) {
            for (var j=0; j < gl_arrPartida[i].length; j++) {
                var arr = gl_arrPartida[i][j].sub_var;
                var index = arr.indexOf(num_var);
                if (index > -1) {
                    //arr.splice(index, 1);
                    gl_arrPartida[i][j].sub_var.splice(index, 1);
                    break;
                }
            }
        }
        var html = creaHtml();
    $("#div_html_partida").html(html);
    // para que se vea el elemento primera jugada de la partida automaticamente cuando la variante es larga
    var elmnt = document.getElementById("Var0Mv1");
    elmnt.scrollIntoView(true); // se muestra arriba
    // como la variante ya no existe, nos ponemos en la primera jugada de la partida
    var v = gl_arrPartida[0][1];
    muestra_jugada(v.idx_jug, v.san, v.uci, 
                    v.num_jug, v.turno, v.fen, 
                    v.nag, v.comen, v.sub_var, v.profundidad);
    }
});
// *********** fin de los botones ****************
