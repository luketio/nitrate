# Nitrate


Name: Luke Tong

## Project description
This is a project similiar to David Chen's [pingmote](https://github.com/dchen327/pingmote) project, but written in Rust and web technologies.

I decided to implement this project with Tauri and Rust, meaning it's fast and lightweight compared to a lot of other similiar cross platform desktop app libraries. It also means I can design the UI using HTML, CSS, and JS frameworks, so while the graphics are rendered the same way as a normal website, rather than opening it with a browser like Chrome, Tauri uses WebViews to render it locally. The result is a fast, good looking, and easy to design UI that doesn't require an internet connection to use. It also looks the same on every platform.

## How to execute
.dmg files are available for Mac in releases.

Click [here](https://github.com/luketio/nitrate/releases/download/v1.0.0/Nitrate_1.0.0_aarch64.dmg) to download the dmg, then run it.

Install it like a normal dmg file and run Nitrate. Because it's not a signed app it may not work so you might have to go to settings to allow running of unsigned applications

First, run the application once.

Afterward, to add emojis, go to `~/Library/Application Support/Nitrate/emojis` and add any sort of picture that you want to use as an emoji.

Then, in the app, click "refresh", and resized emojis should appear, and after clicking on any emoji you should be able to paste it anywhere you want.
