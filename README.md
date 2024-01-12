In the project the server and client parts of the web application are written in Rust (collected in one workspace).    
The work is in progress.       
It is to be an application for society of members interested in houseplants.    
After registration and login members can read and write about different houseplants, add their own records about plants they care at home or are interested in.   
Something like a mini blogs with focus on house plants. 

I use [Actix](https://actix.rs/) Rust web framework for server and client parts,    
[Tera Templates](https://keats.github.io/tera/docs/) for HTML responses formation,     
[PostgreSQL](https://www.postgresql.org/) as database. 

In general,  I follow the approach from the very good books: [The Rust Programming Language; Final Project: Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) and    
[Prabhu Eshwarla, Rust Servers, Services, and Apps](https://www.manning.com/books/rust-servers-services-and-apps),      
[Prabhu Eshwarla, Rust Servers, Services, and Apps, GitHub code of the book](https://github.com/peshwar9/rust-servers-services-apps)

The Postgres DB is to be installed.   
Run     
```
sudo service postgresql restart
```     
to be sure the DB is running.      
Create two databases :   
```
web_client_db_a
```    
for clients side and   
```
houseplants_a
```     
for server.      
Create user 
```
truuser
```      
with password    
```    
a_password
```     
(for simplicity same for both databases).   
Run the 
```
db_scripts
```   
from 
```
server_modules
```   
and 
```
client_modules
```   
to populate the databases (change the placeholders path.to.file..... to real paths).   
```
psql -U truuser -d houseplants_a < path.to.file_plants_and_members.sql
```
and  
```
psql -U truuser -d  < path.to.file_user.sql
``` 

Create .env file (do not forget to gitignore it) in 
```
houseplants-client
```   
folder with the content (example)
```
HOST_PORT=127.0.0.1:8080
SERVER_PORT=127.0.0.1:3000
DATABASE_URL=postgres://truuser:a_password@127.0.0.1:5432/web_client_db_a
```

Create .env file (do not forget to gitignore it) in 
```
houseplants-server
```     
folder with the content (example)
```
SERVER_PORT=127.0.0.1:3000
DATABASE_URL=postgres://truuser:a_password@127.0.0.1:5432/houseplants_a
```
   
In the configuration server is running on 
```
localhost:3000
```  
and  client on 
```
localhost:8080
```

From the root folder for client run   
```
cargo run --bin start-client
```     
and from the root folder for server run    
```
cargo run --bin start-server
```      
Then visit the 
```
localhost:8080/home/
```    
and    
```
localhost:3000/home
```    

The client interface allow at the moment to navigate between (almost all) pages.    
The routes the ```routes.rs``` files in ```houseplants-client```   and ```houseplants-server``` define the routes/functionality of the app resources.    

TODO: logout 
TODO: better custom error handling   
TODO: more views from DB     
TODO: custom views for members 
TODO: use middleware to secure custom views      
TODO: possibility to add and view interesting facts about plants, which are randomly selected from DB   
TODO: possibility for members to create new interesting facts       
TODO: possibility for members to make comments  
TODO: dockerize the app 
TODO: the app deployment   
TODO: massive use of macros in templates 
TODO: the work is in progress 