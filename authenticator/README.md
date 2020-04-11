# Authenticator

This is the server that deals with authenticating the user and
changing its password.

# Environment Variables

The program has 2 variables that can be set at server launch. They are when
launching the app
- Linux: 
    - `<var_name1>=val1 <var_name2>=val2 cargo run`, with as many variables. 
- Windows
    - `$env:<var_name1>='val1'`, a command for each variable to set. If value
    is a number, apostrophes(') are omitted (hint: all are strings).
    - `cargo run`

The variables are:
1. `ROCKET_ENV` is the environment of the server. More info about
changes to server behaviour are found in `Rocket.toml` file. They can be:
    - `development` or `dev`
    - `staging` or `stage`
    - `production` or `prod`
    

2. `KEY` is the key for the validation of the JWT.

# Authorization

Authorization is made by the JWT which has a validation period of **15 minutes**.