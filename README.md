# Karaoke
Karaoke playlist website

This project is aimed at doing a website for providing a list of song for karaoke and allowing anyone to chose the song he wants to sing

The song are loaded from a Google Sheets through google API

The whole project is made in rust with a backend using Actix and a frontend using Yew and hosting through shuttle.rs


To launch the project localy :
    1. Start the backoffice : cd backoffice && cargo shuttle run
    2. Start the frontoffice : cd frontoffice && cargo run
    3. Go to localhost:8080
