# Backend

![crab pet](https://i.imgur.com/LbZJgmm.gif)

# Compile & Run

> [!IMPORTANT]
> All code must be compiled with rust nightly.
> To use nightly run command `rustup override set nightly` before `cargo build`.

There will be 2 main folders in the root directory, **authenticator** and **accountsprovider**. Compilation is made using the command `cargo build` in the specific folder. The executable is built in *<dir_name>/target/debug/* with the name of the directory. It can be run dirrectly or by `cargo run`.

## Environment Variables

Setting variables:

- Linux
    - `<key1>=<val1> cargo run`
    - there can be as many combinations `<key1>=<val1>` before the cargo command.

- Windows
    - `$env:<key1>='<val1>'`, one variable per command,
    - `cargo run`

Variables that can be set are:

1. `ROCKET_ENV` is the environment of the server. The changes made by this setting are found int the `Rocket.toml` file. Values can be:
    - `development` or `dev`
    - `staging` or `stage`
    - `production` or `prod`

2. `KEY` is the key with which to encode and decode the JWT token. Value is a string.

3. Database Location which has key `AUTH_DB` for the **authenticator** and `ACCT_DB` for the **accountsprovider**. Value is of type `<host>:<port>` where *host* is an ip or preconfigured host and *port* is a number ranging from [1, 65535].

4. (Only in **accountsprovider**) `CLEAR` to signal the server to send clear entries in the responses and modify the clear entries. To be used only when debugging.

**Example**: `ROCKET_ENV=dev KEY=secret AUTH_DB=localhost:27017 cargo run` will set the server to run in development mode with the encoding key 'secret' for the JWTs and the database situated at address localhost at port 27017.


# Frontend Unified Communication Key

All information to and from the backend will be given in JSON format. I suggest the following formatting for this (please, it will make my life easier). For any modification, talk to the repo owner.

1. Register
    - Request will be:
    ```
    POST /register HTTP/1.1
    <any other fields>
   
    {
       "username": "genericuser@emailclient.com",
       "name": "Placeholder Dorel",
       "password": "notarealpasswordjustthehash"
       "e_dek": "SuchSecurity",
       "i_kek": "MuchEncryption",
    }
               
    ```
    - Response will be:
    ```
    HTTP/1.1 200 OK
    <any other fields>
    ```

2. Authenticate
    - Request will be: 
    ```
    POST /login HTTP/1.1
    <any other fields>
     
    {
       "username": "genericuser@emailclient.com",
       "password": "notarealpasswordjustthehash"
    }
           
    ```
   - Response will be:
   ```
   HTTP/1.1 200 OK
   Content-Type: application/json
   Content-Length: <body-length>
   <any other fields>
   
   { 
        "name": "Placeholder Dorel",
        "session_timer": 15,
        "e_dek": "SuchSecurity",
        "i_kek": "MuchEncryption",
        "jwt": "<token>" 
   }
   ```
  
3. Modify Master Password
    - Request will be: 
    ```
    POST /modifypassword HTTP/1.1
    Authorization: Bearer <jwt_token>
    <any other fields>
    
    {
        "username": "genericuser@emailclient.com",
        "old_password": "notarealpasswordjustthehash",
        "new_password": "newpasswordhash"
    }
           
    ```
   - Response will be:
   ```
   HTTP/1.1 200 OK
   <any other fields>
   ```

4. Modify Email / Name / Session Timer
    - Request will be: 
    ```
    POST /modifyacctdata HTTP/1.1
    Authorization: Bearer <jwt_token>
    <any other fields>
    
    {
        "field_name": <field_name>,
        "new_value": <new_value>
    }      
    ```
   - Response will be:
   ```
   HTTP/1.1 200 OK
   <any other fields>
   ```

   The `field_name` can be:
   - `"email"`
   - `"name"`
   - `"session_timer"`
   
   The field `new_value` holds a string, even the session timer.

5. Get Accounts List
    - Request will be:
    ```
    GET /accounts HTTP/1.1
    Authorization: Bearer <jwt_token>
    <any other fields>
    ```
    - Response will be:
    ```
    HTTP/1.1 200 OK
    Content-Type: application/json
    Content-Length: <body-length>
    <any other fields>
    
    {
        "accounts": [
            {
                "id": "random_string1",
                "site": "site1.com",
                "username": "ceva1",
                "password": "altceva2",
                "favorite": true
            },
            {
                "id": "random_string2",
                "site": "site2.com",
                "username": "ceva2",
                "password": "altceva1",
                "favorite": false
            }
        ]
    }
    
    ```
   
6. Add/Modify Account Information
    - Request will be: 
    ```
   PUT /modifyaccount HTTP/1.1
   Authorization: Bearer <jwt_token>
   <any other fields>
   
   {
       "id": "random_string",
       "site": "bestnsfwsite.com",
       "username": "genericuser@emailclient.com",
       "password": "nohackerpls",
       "favorite": false
   }
   ```
   - Response for **adding new account** will be: 
   ```
   HTTP/1.1 201 Created
   <any other fields>
   ```
   - Response for **modifying account info** will be: 
    ```
    HTTP/1.1 200 OK
    <any other fields>
    ```
   
7. Delete Account Information
   - Request will be: 
   ```
   DELETE /modifyaccount HTTP/1.1
   Authorization: Bearer <jwt_token>
   <any other fields>
  
   {
        "id": "random_string",
        "site": "bestnsfwsite.com",
        "username": "genericuser@emailclient.com",
        "password": "nohackerpls",
        "favorite": false
   }
   ```
    - Response will be: 
    ```
    HTTP/1.1 200 OK
    <any other fields>
    ```
    
8. Error Responses
    - Incorrect Body Format
    ```
    HTTP/1.1 400 Bad Request
    <any other fields>
    ```
   - Bad Master Password
    ```
    HTTP/1.1 401 Unauthorized 
    <any other fields>
    ```
    - Access Page Without Authentication / Wrong Old Password
    ```
    HTTP/1.1 403 Forbidden 
    <any other fields>
    ```
   - Unknown request page
   ```
   HTTP/1.1 404 Not Found
   <any other fields>
   ```
   - Incorrect JSON Format
    ```
    HTTP/1.1 422 Unprocessable Entity
    <any other fields>
    ```
    - Already Registered User
   ```
   HTTP/1.1 498 Already Registered User
   ```
   - Unregistered User
   ```
   HTTP/1.1 499 Unregistered User
   ```
   - Rust backend panicked
   ```
    HTTP/1.1 500 Internal Server Error
    <any other fields>
   ```
    - Could not be bothered to implement yet
   ```
   HTTP/1.1 501 Not Implemented
   <any other fields>
   ```
   
## JWT

It is valid for 15 minutes.
