# soundshouter - play sounds whenever needed

Wer kennt das nicht - Du sitzt mit deinen Dudes bei einer gepflegten Partie Mensch-Ärgere-Dich, Du rasierst gerade zwei gegnerische Spielfiguren übern Rasen, und denkst Dir "ULTRAKILL". Wenn jetzt der zugehörige Sound in klassischer Quake 3 Manier aus den Lautsprechern käme, um deinen Sieg zu zementieren - ein Traum würde in Erfüllung gehen! Gut, dass wir hier den soundshouter mit genau dieser Funktionalität entwickeln. 

# Features

## Playback von beliebigen lokalen Sounddateien 

Der Soundshouter ist darauf optimiert, kurze Audiosequenzen über lokal angeschlossene Lautsprecher abzuspielen. 

## Automatisches Einlesen der Soundfiles

Spare dir das händische Zuweisen von Buttons zu Soundfiles. Erstelle lieber eine Ordnerstruktur mit Audiodateien (mp3/ogg/wav), lege Sie dem soundshouter vor und er erkennt selbstständig, welche Sounds in welcher Hierarchie auswählbar sind

## Multi-Interface Konzept

Das Programm soll folgende Eingabemethoden unterstützen:
* Eingabe über ein Web-Interface (klick button to play sound)
* Eingabe über ein Bedienfenster (klick button to play sound)
* Anbindung an MQTT (select / play sound, report playback to MQTT channel)



