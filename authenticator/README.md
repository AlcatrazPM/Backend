# Authenticator

This is the server that deals with authenticating the user and
changing its password.

# Environment Variables

The program has 2 variables that can be set at server launch:
1. `ROCKET_ENV` is the environment of the server. More info about
changes to server behaviour are found in `Rocket.toml` file. They can be:
    - development
    - staging
    - production
    

2. `KEY` is the key to verify JWT.

# Authorization

Authorization is made by the JWT which has a validation period of **3 minute**.