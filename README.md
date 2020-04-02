# Backend

![crab pet](https://i.imgur.com/LbZJgmm.gif)

Link to [roadmap](https://www.dropbox.com/scl/fi/rsyh6p0wzddm9tpr9sc9b/AlcatrazPM-Roadmap.paper?dl=0&rlkey=btnjbf5vg4oaz1dtfx8xx6854).

# Compile & Run

Compilation is made using the command `cargo build` in the root folder. The executable
is built in */target/appserver* (or *\target\appserver.exe* in Windows). It can be either
run directly or by `cargo run`.
The server will start and listen on port 8082 (need to change this) and any requests
made to it will be printed to the console (for debugging).

# Input/Output Request Format

All information to and from the backend will be given in JSON format. I suggest the
following formatting for this (please, it will make my life easier). For any 
modification, talk to the repo owner.

1. Register
    - Request will be: **TODO**
    - Response will be: **TODO**

2. Authenticate
    - Request will be: 
    ```
           POST /authenticate HTTP/1.1
           <any other fields>
           
           {
               "Username": "vlad_e_hispter@gmail.com",
               "Password": "notarealpasswordjustthehash"
           }
           
    ```
   - Response will be: **TODO**


3. Get Accounts List
    - Request will be: **TODO**
    - Response will be:
    ```
    POST /accountslist HTTP/1.1
    <any other fields>
    
    {
        "Accounts": [
            {
                "Username": "ceva1",
                "Password": "altceva2",
                "Id": "id1"
            },
            {
                "Username": "ceva2",
                "Password": "altceva1",
                "Id": "id2"
            }
        ]
    }
    
    ```
   
4. Add/Remove/Modify Account Information
    - Request will be: 
    ```
   POST /modifyaccount HTTP/1.1
   <any other fields>
   
   {
       "Action": "add",
       "Site": {
           "Username": "vlad_e_hipster@gmail.com",
           "Password": "nohackerpls",
           "Id": "faranumar"
       }
   }
   ```
   The `Action` field can be: `add`, `remove`, `modifiy`.
   - Response will be: **TODO**
   
    **Note**: The examples above show the JSONs as easily readable, in he proper
    requests, I will prefer all to be in one line.