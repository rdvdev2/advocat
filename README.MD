# Advocat
No presentis res al [jutge](https://jutge.org) sense abans consultar al teu advocat!

## Que fa l'advocat?
Aquest script és capaç de compilar els programes del jutge i provar-los automaticament. Els tests públics es descarregaràn de forma automàtica i es comprovaran. Si el problema demana un procediment en comptes d'un programa, es descarregarà el procediment `main()` utilitzat per el jutge per a evaluar el programa, evitant que hagis d'escriure tu mateix aquest mètode que el jutge no evaluarà.

## Requisits
- bash (o derivats)
- p1++
- grep
- git

## Instruccions d'ús
Guarda el teu treball com a `main.cc` a una carpeta anomenada com la ID del problema del jutge (p.ex. `P90615_ca/main.cc`). Executa `advocat` al directori i el script automaticament complilarà i provarà el teu programa. Tingues en compte que si la ID del problema comença amb 'X' el problema és privat i no és possible descarregar els arxius automàticament. En aquest cas, el programa et donarà totes les instruccions necessàries per a poder descarregar els arxius manualment i possar-los a disposició del script.

## Instruccions d'instalació / actualització
Descarrega el repositori a una ubicació fàcil de recordar:
``` Bash
cd ~
git clone https://github.com/rdvdev2/advocat
```
Afegeix la comanda `advocat` al entorn de forma automàtica, modificant el `.bashrc`, `.zshrc` o similars:
``` Bash
# Fitxer: ~/.bashrc o ~/.zshrc
# Afegeix les següents línies al final del fitxer

# Afegeix el advocat a l'entorn
source ${HOME}/advocat/advocat.sh
```
El proper cop que iniciis la terminal el script `advocat` estarà disponible!

Per actualitzar el programa només et caldrà actualitzar el repositori:
``` Bash
cd ~/advocat
git pull
```
*Nota: Aquest procés de instalació i actualització serà automatitzat en una futura versió del programa*

## Com afegir tests a un problema
Alguns problemes al jutge no tenen tests públics, o potser vols afegir al teu joc de proves un test privat que ha fallat o altres casos inventats per tu. En aquests casos, l'advocat pot llegir els tests que guardis a la mateixa carpeta que el teu ```main.cc```. Cada test consta de dos arxius, on ```#``` és el nombre del test (començant per 1):
- ```sample-#.inp``` (o ```sample.inp``` en cas d'haver-hi un únic test): Contè l'entrada del programa
- ```sample-#.cor``` (o ```sample.cor``` en cas d'haver-hi un únic test): Contè la sortida correcta del programa per a l'entrada corresponent

És important vigilar les línies en blanc al final del arxiu ```.cor```, ja que si hi ha una línia en blanc al final del arxiu el test només es considerarà correcte si el programa també produeix una línia en blanc. En el cas d'escriure tests per a un problema de procediment (sense ```main()```) es recomana comprovar l'arxiu ```~/.advocat/ID_PROBLEMA/main.cc``` per saber com es llegirà l'entrada.