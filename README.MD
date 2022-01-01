# Advocat
No presentis res al [jutge](https://jutge.org) sense abans consultar al teu advocat!

## Que fa l'advocat?
Aquest programa és capaç de compilar els programes del jutge i provar-los automaticament. Els tests públics es descarregaràn de forma automàtica i es comprovaran. Si el problema demana un procediment en comptes d'un programa, es descarregarà el procediment `main()` utilitzat per el jutge per a evaluar el programa, evitant que hagis d'escriure tu mateix aquest mètode que el jutge no evaluarà.

## Requisits
- g++
- diff
- python (build)
- ninja (build)

## Instruccions d'ús
Guarda el teu treball com a `main.cc` a una carpeta anomenada com la ID del problema del jutge (p.ex. `P90615_ca/main.cc`). Executa `advocat` al directori i el programa automaticament complilarà i provarà el teu programa. Tingues en compte que si la ID del problema comença amb 'X' el problema és privat i no és possible descarregar els arxius automàticament. En aquest cas, el programa et donarà totes les instruccions necessàries per a poder descarregar els arxius manualment i possar-los a disposició del advocat.

## Instruccions d'instalació / actualització
Clona el repositori:
``` Bash
git clone https://github.com/rdvdev2/advocat
```
Compila el programa:
``` Bash
cd advocat
chmod +x configure.py
./configure.py
ninja
```
Instala el programa:
``` Bash
ninja install
```
Recorda afegir el directori d'instalació al path:
``` Bash
# Al final del teu .bashrc / .zshrc
export PATH=$PATH:$HOME/.advocat/program
```
Si tornes a obrir la terminal o recarregues el fitexer `.bashrc` / `.zshrc` el programa ja estarà disponible.

Si necessites actualitzar el programa, descarrega els canvis i torna a compilar i instalar:
``` Bash
git pull
./configure.py
ninja
ninja install
```

## Com afegir tests a un problema
Alguns problemes al jutge no tenen tests públics, o potser vols afegir al teu joc de proves un test privat que ha fallat o altres casos inventats per tu. En aquests casos, l'advocat pot llegir els tests que guardis a la mateixa carpeta que el teu ```main.cc``` i a les seves subcarpetes. Cada test consta de dos arxius, on ```#``` és el nombre del test (començant per 1):
- ```sample-#.inp``` (o ```sample.inp``` en cas d'haver-hi un únic test): Contè l'entrada del programa
- ```sample-#.cor``` (o ```sample.cor``` en cas d'haver-hi un únic test): Contè la sortida correcta del programa per a l'entrada corresponent

És important vigilar les línies en blanc al final del arxiu ```.cor```, ja que si hi ha una línia en blanc al final del arxiu el test només es considerarà correcte si el programa també produeix una línia en blanc. En el cas d'escriure tests per a un problema de procediment (sense ```main()```) es recomana comprovar l'arxiu ```~/.advocat/ID_PROBLEMA/main.cc``` per saber com es llegirà l'entrada.

## Llicències del codi font
Les plantilles de la carpeta `templates/`, així com els procediments per a compilar els programes són adaptats del repositori [jutge-org/jutge-driver-std](https://github.com/jutge-org/jutge-driver-std).