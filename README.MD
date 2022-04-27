# Advocat
No presentis res al [jutge](https://jutge.org) sense abans consultar al teu advocat!

## Que fa l'advocat?
Aquest programa és capaç de compilar els problemes del jutge i provar-los automàticament. Els tests es descarregaràn de
forma automàtica i es comprovaran. Si el problema demana un procediment en comptes d'un programa, es descarregarà el
procediment `main()` utilitzat pel jutge per a avaluar el programa, evitant que hagis d'escriure tu mateix aquest
mètode que el jutge no avaluarà.

## Requisits
- g++
- curl
- rust (build)
- cargo (build)

## Instruccions d'ús
Guarda el teu treball com a `main.cc` a una carpeta anomenada com la ID del problema del jutge
(p.ex. `P90615_ca/main.cc`). Executa `advocat` al directori i el programa automàticament complilarà i provarà el teu
programa. Tingues en compte que si la ID del problema comença amb 'X' el problema és privat i no és possible descarregar
els arxius sense iniciar sessió. Pots iniciar sessió al jutge a través de l'advocat modificant l'arxiu de configuració
`~/.config/advocat/config.ini` per incloure els següents paràmetres:
``` ini
[auth]
email = EL_TEU_CORREU
password = LA_TEVA_CONTRASENYA
```

## Instruccions d'instal·lació / actualització
Per a instal·lar o actualitzar el programa:
``` shell
cargo install --path https://github.com/rdvdev2/advocat
```
Si mai has instal·lat programes mitjançant `cargo install`, afegeix la ruta d'instal·lació al path:
``` shell
# Al final del teu .bashrc / .zshrc
export PATH=$PATH:$HOME/.cargo/bin
```
Si tornes a obrir la terminal o recarregues el fitxer `.bashrc` / `.zshrc` el programa ja estarà disponible.

## Com afegir tests a un problema
Alguns problemes al jutge no tenen tests públics, o potser vols afegir al teu joc de proves un test privat que ha fallat
o altres casos inventats per tu. En aquests casos, l'advocat pot llegir els tests que guardis a la mateixa carpeta que
el teu `main.cc` i a les seves subcarpetes. Cada test consta de dos arxius, amb el mateix nom i diferent extensió:
- `NOM_TEST.inp`: Conté l'entrada del programa
- `NOM_TEST.cor`: Conté la sortida correcta del programa per a l'entrada corresponent

És important vigilar les línies en blanc al final de l'arxiu `.cor`, ja que si hi ha una línia en blanc al final de
l'arxiu el test només es considerarà correcte si el programa també produeix una línia en blanc. En el cas d'escriure
tests per a un problema de procediment (sense `main()`) es recomana comprovar l'arxiu 
`~/.cache/advocat/ID_PROBLEMA/main.cc` per saber com es llegirà l'entrada.

## Llicències del codi font
Les plantilles de la carpeta `templates/`, així com els procediments per a compilar els programes són adaptats del
repositori [jutge-org/jutge-driver-std](https://github.com/jutge-org/jutge-driver-std).

## TODO
Tot i que l'advocat és un programa funcional, encara hi ha algunes característiques pendents d'implementar:
- Compatibilitat amb altres compiladors (actualment s'utilitza el `p1++`)
- Compatibilitat amb altres llenguatges (actualment només es processen programes en C++)
- Execució dels tests en paral·lel
- Traducció al català i el castellà (actualment el programa és en anglès)
- Interacció directa amb el jutge:
  - Descàrrega d'enunciats
  - Publicació de resolucions i mostra del veredicte
  - Navegació de llistes
  - Descàrrega de solucions prèvies
- Càlcul de mètriques (ccn, cl, diff...)
- Paràmetres per modificar el comportament del programa (eliminar la cache, no descarregar arxius, utilitzar arxius
diferents de main.cc)
- Més opcions de configuració (similars als paràmetres)

Aquesta llista no està en ordre de prioritat, i qualsevol contribució és benvinguda.
