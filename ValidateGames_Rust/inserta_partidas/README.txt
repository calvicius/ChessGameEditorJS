Uso / Use :

cargo run "pgns/nombre_fichero.pgn"
inserta_partidas.exe "pgns/nombre_fichero.pgn"

Explicacón / Explanation:

Lee un archivo de partidas de ajedrez en formato PGN.
Lee y valida los movimientos de cada partida incluida en el fichero.
La validación de los movimentos se hace con una versión reducida de https://github.com/calvicius/calvichess
Crea una estructura interna para, posteriormente, visualizar las partidas en un formato gráfico.

Cuando haya una partida errónea, ésta se mutila sin "ruidos", y se continúa con las partidas sucesivas.

Por último, las partidas se graban en una base de datos en SQLITE (../sqlite_database), y tambien se exportan las partidas en formato JSON (../html/partidas_json.js)

Hay partidas de ejemplo en el directorio ../pgns/nombre_fichero

---

Read a file of chess games in PGN format.
Read and validate the movements of each game included in the file.
The validation of the movements is done with a reduced version of https://github.com/calvicius/calvichess
An internal structure is created to later visualize the games in a graphic format.

When there is an erroneous game, it is mutilated without "noises", and the successive games are continued.

Finally, the games are recorded in a database in SQLITE (../sqlite_database), and the games are also exported in JSON format (../html/partidas_json.js)

There are sample games in the ../pgns/filename directory