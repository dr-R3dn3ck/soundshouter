# soundshouter - play sounds whenever needed

Wer kennt das nicht - Du sitzt mit deinen Dudes bei einer gepflegten Partie Mensch-Ärgere-Dich, Du rasierst gerade zwei gegnerische Spielfiguren übern Rasen, und denkst Dir "ULTRAKILL". Wenn jetzt der zugehörige Sound in klassischer Quake 3 Manier aus den Lautsprechern käme, um deinen Sieg zu zementieren - ein Traum würde in Erfüllung gehen! Gut, dass wir hier den soundshouter mit genau dieser Funktionalität entwickeln. 

# Features 

## Playback von beliebigen lokalen Sounddateien 

Der Soundshouter ist darauf optimiert, kurze Audiosequenzen über lokal angeschlossene Lautsprecher abzuspielen. 

## Automatisches Einlesen der Soundfiles

Spare dir das händische Zuweisen von Buttons zu Soundfiles. Erstelle lieber eine Ordnerstruktur mit Audiodateien (mp3/ogg/wav), lege Sie dem soundshouter vor und er erkennt selbstständig, welche Sounds in welcher Hierarchie auswählbar sind

## Multi-Interface Konzept

Das Programm soll folgende Eingabemethoden unterstützen:
* [x] Eingabe über ein Web-Interface (click button to play sound)
* [ ] Eingabe über ein Bedienfenster (click button to play sound)
* [ ] Anbindung an MQTT (select / play sound, report playback to MQTT channel)

# Installation

## Soundshouter Server 

Der Server enthält eine API, Audiowiedergabe und MQTT queue.
Die Webapp wird eingebettet, und durch den Server ausgeliefert.

[rustup / rust installieren](https://rustup.rs/)

Abhängigkeiten
* debian: libsqlite3-dev
* fedora: sqlite-devel (sqlite version 3 wird benötigt)
* cpal 
* debian: libasound2-dev 
* fedora: alsa-lib-devel

*Kompilieren*
```bash
cargo build
```

*Sounds importieren*
```bash
soundshouter import <sounds ordner>
```

*Server starten (Entwicklungsumgebung)*
```bash
soundshouter serve
```

## Webapp

[npm Installieren](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
```bash
# dependencies Installieren
npm install

# build
npm run build

# test server starten
npm run dev
```

```bash
# dev container
{
    "name": "Ubuntu",
    "image": "mcr.microsoft.com/devcontainers/base:jammy",
    "forwardPorts": [5173, 8000],  // Forward both port 5173 (for frontend) and port 8000 (for Rocket server)
    "workspaceFolder": "/workspaces/Projects",
    "postCreateCommand": "/bin/bash -c '\
        sudo apt update && \
        sudo apt install -y pkg-config libasound2-dev libsqlite3-dev && \
        if [ -d \"soundshouter\" ]; then rm -rf soundshouter; fi && \
        git clone --branch unified --single-branch https://github.com/dr-R3dn3ck/soundshouter.git soundshouter && \
        cd soundshouter/soundshouter-vite-webapp && \
        echo \"📁 Entered soundshouter-vite-webapp\" && ls -la && \
        curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash && \
        export NVM_DIR=\"$HOME/.nvm\" && \
        [ -s \"$NVM_DIR/nvm.sh\" ] && . \"$NVM_DIR/nvm.sh\" && \
        nvm install --lts && \
        npm install -g vite && \
        npm install && \
        cd ../soundshouter-vite-webapp && \
        npm install && \
        npm run build && \
        cd ../soundshouter && \
        echo \"📁 Entered the soundshouter directory\" && ls -la && \
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && \
        source $HOME/.cargo/env && \
        rustup install nightly && \
        rustup default nightly && \
        cargo build'"
    ,
    "ports": [
        "8000:8000"  // Explicitly publish the 8000 port for Rocket server
    ]
}
```

```bash
#run server with
ROCKET_ADDRESS=0.0.0.0 target/debug/soundshouter serve

```

