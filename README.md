# Pea Shooter
This is our Rust Project for CS410P. It is a topdown shooter where you have to survive as long as you can against waves of enemies. 

## How To Play
How to play. Move using WASD and try to avoid the enemies. You can fire a projectile by clicking in the game arena with the mouse. Survive as long as you can!

## Engine
This project is using the Amethyst engine, version 0.10.0

## Building
Building on Ubuntu

The following packages need to be installed:
```
$ apt install gcc pkg-config openssl libasound2-dev cmake build-essential python3 libfreetype6-dev libexpat1-dev libxcb-composite0-dev
```

Building on Windows

The vs build tools needs to be installed if you are using vscode.

## How to run
Make sure cargo is installed locally using rustup. Clone the repo, and call:
```
 cargo run
```

## Sources

We used an amethyst project to base our project off of. That project can be found: https://github.com/trsoluti/space_shooter

Some of our code was reused from this project. We also took inspiration for our file structure from this project.
