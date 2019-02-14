$(document).ready(function(){
    
	function clickLanzaMotorBtn(event) {
        var motor ='./js_motor/stockfish.js';
        var stockfish = new Worker(motor);
		var mensaje = document.querySelector("#mensaje");
		var posic_motor = 'position fen ' + $('#fen').text();
        //var partida1 = new Chess($('#fen').text());
        
		stockfish.postMessage(posic_motor);
		//stockfish.postMessage('go depth 20');
		//aparte de la manera con postMessage lo podemos hacer asi
		function uciCmd(cmd) {
			stockfish.postMessage(cmd);
		}
        if (event.data.param1 == "start") {
            uciCmd('go depth ' + profundidad);
        }
        else if (event.data.param1 == "stop") {
            console.log(20, "parando stockfish ...");
            uciCmd("stop");
            uciCmd("quit");
            stockfish.terminate();
        }
        
		stockfish.onmessage = function(event) {
			//NOTA: los Web Workers meten la respuesta en un objeto.
			//console.log(event.data ? event.data : event);
		  var linea_analisis = event.data;
		  var array_salida = linea_analisis.split(" ");
		  var evaluacion = "";
		  var pieza = "";
		  var evaluacion = "";
		  var partida1 = new Chess($('#fen').text());

		  if(linea_analisis == 'uciok') {
            $("#eval").html("Motor cargado");
          } else if(linea_analisis == 'readyok') {
            $("#eval").html("Motor listo");
          } 
		  var match = linea_analisis.match(/^bestmove ([a-h][1-8])([a-h][1-8])([qrbk])?/);
          
          if(match) {
			var movim = partida1.move({from: match[1], to: match[2], promotion: match[3]});
            if (!movim){
                stockfish.terminate();
                $("#calculo").val("");
                return;
            }
            var jugada = movim.san;
			var turno = partida1.turn();
			var arrayFEN = $('#fen').text().split(" ");
			var numJugada = parseInt(arrayFEN[5]);
			var salida_final = "";
			if (turno == "w") {
				var salida_final = "Mejor jugada : <strong>" + numJugada + " ... " + jugada + "</strong>";  // + " / " + posible + ": " + replica;
			} else {
				var salida_final = "Mejor jugada : <strong>" +numJugada + ". " + jugada + "</strong>";
			}
			$("#mensaje").html('<span class="movim" style="color:red;">' + salida_final + '</span>');
          } else if(match = linea_analisis.match(/^info .*\bdepth (\d+) .*\bnps (\d+)/)) {
            var profun = 'Profundidad: ' + match[1] + ' Nps: ' + match[2];
			$("#mensaje").html(profun);
          }
		  var posicion = linea_analisis.lastIndexOf(' pv');
		  if(posicion != -1){
			  posicion = parseInt(posicion);
			  var porcion = linea_analisis.substring(posicion+4);
			  var x = porcion.split(" ");

			  for(var i=0; i<x.length; i++){
				var desde = x[i].substring(0, 2);
				var hasta = x[i].substring(2, x[i].length);
				var corona = "";
				if (hasta.length >2){
					corona = hasta[2];
					hasta = hasta.substring(0, 2);
				}
				var movim = partida1.move({from: desde, to: hasta, promotion: corona});
				//var jugada = movim.san;
				var piensa = partida1.pgn();
				piensa1 = piensa.split("\n");
				if (piensa1[3] !== 'undefined'){
                    if (piensa1[0][0] == "[") {
                        $("#calculo").html('<span class="movim" style="color:OrangeRed;">' + piensa1[3] + '</span>');
                    }
                    else {
                        $("#calculo").html('<span class="movim" style="color:OrangeRed;">' + piensa1[0] + '</span>');
                    }
				}
			  }
		  }
          if(match = linea_analisis.match(/^info .*\bscore (\w+) (-?\d+)/)) {
            var score = parseInt(match[2]) * (partida1.turn() == 'w' ? 1 : -1);
            if(match[1] == 'cp') {
				var puntu = (score / 100.0).toFixed(2);
				evaluacion = "Eval." + ": " + puntu;
				$("#eval").html(evaluacion);		
            } else if(match[1] == 'mate') {
                var jaqueMate = '# Mate en: ' + score;
				$("#eval").html(jaqueMate);	
            }
            if(match = linea_analisis.match(/\b(upper|lower)bound\b/)) {
                evaluacion = ((match[1] == 'upper') == (partida1.turn() == 'w') ? '<= ' : '>= ') + evaluacion
            }
          }
		};
	}
    
    var profundidad = 20;
    $("select.habilidad").change(function(){
        var valor = $(".habilidad option:selected").val();
        profundidad = parseInt(valor);
    });
    
    
    
    
	$('#LanzaMotorBtn').click({param1:"start"}, clickLanzaMotorBtn);    //on('click', clickLanzaMotorBtn({param1:"start"}));
    $('#ParaMotorBtn').click({param1:"stop"}, clickLanzaMotorBtn);         //on('click', clickLanzaMotorBtn({param1:"stop"}));
});