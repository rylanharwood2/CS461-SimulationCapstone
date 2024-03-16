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