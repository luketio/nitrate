# Nitrate


Name: Luke Tong

## Project description
This is a project similiar to David Chen's [pingmote](https://github.com/dchen327/pingmote) project, but written in Rust and web technologies.

I decided to implement this project with Tauri and Rust, meaning it's fast and lightweight compared to a lot of other similiar cross platform desktop app libraries. It also means I can design the UI using HTML, CSS, and JS frameworks, so while the graphics are rendered the same way as a normal website, rather than opening it with a browser like Chrome, Tauri uses WebViews to render it locally. The result is a fast, good looking, and easy to design UI that doesn't require an internet connection to use. It also looks the same on every platform.

![Screenshot](https://raw.githubusercontent.com/luketio/nitrate/main/docs/screenshot.png)

## How to execute
.dmg files are available for Mac in releases.

Click [here](https://github.com/luketio/nitrate/releases) to download the most recent dmg, then run it.

Install it like a normal dmg file and run Nitrate. Because it's not a signed app it may not work so you might have to go to settings to allow running of unsigned applications

First, run the application once.

Afterward, to add emojis, go to `~/Library/Application Support/Nitrate/emojis` and add any sort of picture that you want to use as an emoji.

Then, in the app, click "refresh", and resized emojis should appear, and after clicking on any emoji you should be able to paste it anywhere you want.

## Data Types
I used the "Vec" type in Rust, which is the equivalent of an ArrayList.

## Methods
clipboard::copy_image, which is the code to copy all the bytes of an image onto the system clipboard

fs::get_filenames, gets all the filenames of the input emojis or resized emojis

imageproc::resize_all, resizes all unresized input emojis into resized emojis

imageproc::resize, resizes one image, its a private function

imageproc::get_image_data, get all bytes of an image, and convert it to a Base64 String which can be displayed on the frontend

## Data needed
Use custom images and put them in `~/Library/Application Support/Nitrate/emojis`, then click refresh.

## Limitations
Hotkeys don't work because Mac doesn't allow this program to constantly listen for hotkeys no matter if the window is active or not.

## Extra Help
None

## Serious Problems
None
