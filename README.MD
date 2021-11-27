# Advocat
No presentis res al [jutge](https://jutge.org) sense abans consultar al teu advocat!

## Que fa l'advocat?
Aquest script és capaç de compilar els programes del jutge i provar-los automaticament. Els tests públics es descarregaràn de forma automàtica i es comprovaran. Si el problema demana un procediment en comptes d'un programa, es descarregarà el procediment `main()` utilitzat per el jutge per a evaluar el programa, evitant que hagis d'escriure tu mateix aquest mètode que el jutge no evaluarà.

## Requisits
- bash (o derivats)
- p1++
- grep

## Instruccions d'ús
Guarda el teu treball com a `main.cc` a una carpeta anomenada com la ID del problema del jutge (p.ex. `P90615_ca/main.cc`). Executa `advocat` al directori i el script automaticament complilarà i provarà el teu programa. Tingues en compte que si la ID del problema comença amb 'X' el problema és privat i no és possible descarregar els arxius automàticament. En aquest cas, el programa et donarà totes les instruccions necessàries per a poder descarregar els arxius manualment i possar-los a disposició del script.

## Instruccions d'instalació
Descarrega el script i activa-hi el permís d'execució:
```Bash
chmod +x advocat.sh
```
Afegeix el programa a l'entorn actual amb:
```Bash
source advocat.sh
```
O, si ho prefereixes, automatitza la tasca afegint la mateixa línia al teu `.profile`:
```Bash
# ~/.profile
# ...

# Afegeix l'advocat a l'entorn
source ~/advocat.sh # Suposant que has guardat l'script a aquesta ubicació
```
El proper cop que iniciis la terminal el script `advocat` estarà disponible sense necessitat d'executar el script manualment!
