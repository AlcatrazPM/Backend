# Backend

![crab pet](https://i.imgur.com/LbZJgmm.gif)

# Important Notice

1. The backend is going in a long redesinging faze, such that the AccountsProvider is no longer
present in the repo. At the moment all effort is going to Authenticator (full feature).
2. The backend responses are not complete and they don't usually work in a browser due to the
CORS policy. Please don't use a browser for requests at the moment. I am working on it and
I am sorry for the inconvenience.

# Compile & Run

There will be 2 main folders in the root directory, authenticator and accountsprovider.
Compilation is made using the command `cargo build` in the specific folder. The executable
is built in */target/<dir_name>* (or *\target\<dir_name>.exe* in Windows). It can be either
run directly or by `cargo run`.
The server will start and listen on port 8082 (need to change this) and any requests
made to it will be printed to the console (for debugging).

# Frontend Unified Communication Key

All information to and from the backend will be given in JSON format. I suggest the
following formatting for this (please, it will make my life easier). For any 
modification, talk to the repo owner.

1. Register
    - Request will be:
    ```
    POST /register HTTP/1.1
    <any other fields>
   
    {
       "Username": "genericuser@emailclient.com",
       "Password": "notarealpasswordjustthehash"
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
       "Username": "genericuser@emailclient.com",
       "Password": "notarealpasswordjustthehash"
    }
           
    ```
   - Response will be:
   ```
   HTTP/1.1 200 OK
   Content-Type: application/json
   Content-Length: <body-length>
   <any other fields>
   
   { "jwt": "<token>" }
   ```
  
2. Modify Master Password
    - Request will be: 
    ```
    POST /modifypassword HTTP/1.1
    Authorization: Bearer <jwt_token>
    <any other fields>
    
    {
        "Username": "genericuser@emailclient.com",
        "OldPassword": "notarealpasswordjustthehash",
        "NewPassword": "newpasswordhash"
    }
           
    ```
   - Response will be:
   ```
   HTTP/1.1 200 OK
   <any other fields>
   ```

3. Get Accounts List
    - Request will be:
    ```
    POST /getaccounts HTTP/1.1
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
        "Accounts": [
            {
                "Username": "ceva1",
                "Password": "altceva2",
                "site": "site1.com"
            },
            {
                "Username": "ceva2",
                "Password": "altceva1",
                "site": "site2.com"
            }
        ]
    }
    
    ```
   
4. Add/Remove/Modify Account Information
    - Request will be: 
    ```
   POST /modifyaccount HTTP/1.1
   Authorization: Bearer <jwt_token>
   <any other fields>
   
   {
       "Operation": "add",
       "Site": {
           "Username": "genericuser@emailclient.com",
           "Password": "nohackerpls",
           "site": "bestnsfwsite.com"
       }
   }
   ```
   The `Action` field can be: `add`, `remove`, `modifiy`.
   - Response will be: 
   ```
   HTTP/1.1 200 OK
   <any other fields>
   ```
   
    **Note**: The examples above show the JSONs as easily readable, in he proper
    requests, I will prefer all to be in one line.
    
5. Error Responses
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
   
