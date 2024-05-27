# CS461-SimulationCapstone

## Setup

This Project requires a FREE 3rd party API KEY in order to work CORRECTLY!

1. Go to this website and signup
https://developers.nextzen.org/
2. Continue with github (use your github account to create an account)
3. Once created and signed in, create a new API key
4. It should be on this website: https://developers.nextzen.org/keys
5. Then go into the cloned repository and create the ".env" file. It should be on the same directory as the "cargo.toml" file.
6. In the ".env" assign your newly created API key to a variable called "Nextzen_API" 
7. It should look somthing like this: Nextzen_API = xxxxxxxxxxxxxxx
8. Done
9. WARNING: The ".env" file should NEVER be pushed onto the repository


## Tutorial
1. Type "cargo run" while in the clone directory containing the cargo.toml file
2. Click on the play button in the center of the menu screen

## Controls
1. Pitch controls: W/S
2. Roll left/right: Q/E
3. Thrust strength: left shift/left ctrl
4. Enable flaps: F
5. Flap controls: arrow keys
6. Lock cursor: space
7. Pause: ESC
8. Camera control: mouse/scroll wheel
9. Enable Directional arrows: G

# Future Project Plans
1. Flesh out UI
   - Text fonts, visuals, and background in start menu
   - visual pause UI when pressing escape

2. Physics
   - Make physics more realistic, primarily the lift force
   - Give the plane a hitbox and add collisions

3. Customization
   - Allow support for changing between different plane models
   - Allow customizable physics osetttings
   - Allow users to change keybinds
     
## Project Structure                

```text
.
├── docs/
├── src/
├── wip/
├── .gitignore
├── LICENSE
└── README.md
```

The `docs/` directory holds documentations

The `src/` directory contains codebase.

The `wip/` directory contains assignment templates that we will modify/update during the year. You can add more files there that support your project (e.g., diagrams, design files, documents, reviews, progress reports, etc.)
