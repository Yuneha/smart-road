# Smart Road

Smart Road is a cursus project in RUST from Zone01. This project is simulation of a cross road intersection wich AV (Autonomous vehicles) are crossing. It simulate the traffic in this intersection avoiding traffic accidents.
At the end of the simulation it will show some statistics about the simulation.

## Prerequisites

- A code editor (ex: vscode, ...)
- A terminal (ex: powershell, wsl, ...)

## Technologies Used

- **[Rust](https://www.rust-lang.org/fr)**

## Installation

Clone the repository and open the folder with your code editor.

## Usage

1.  On a Terminal

    1.  If you are not in the smart-road folder, navigate to **../smart-road** using **cd** command

            cd smart-road

    2.  You probably need to install sdl2, ttf and/or sdl2_image librairies

            Cargo install *library_name*
            example: cargo install sdl2_image

    3.  Use this command line to run the program :

            cargo run

2.  Commands :

    1. Up / Left / Down / Right arrow key to spawn a vehicle to the respective direction.

    2. 'r' key button to spawn a vehicle at a random direction.

    3. space key button to pause the simulation.

    4. 'c' button key to clear / reset the simulation

    5. escape key button to quit the simulation (it will open the stats windows, escape key again to quit)
