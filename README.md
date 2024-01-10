In the project the server and client parts of the web application are written in Rust (collected in one workspace).    
The work is in progress.       
It is to be an application for society of members interested in houseplants.    
After registration and login members can read and write about different houseplants, add their own records about plants they care at home or are interested in.   
Something like a mini blogs with focus on house plants. 

I use [Actix](https://actix.rs/) Rust web framework for server and client parts,    
[Tera Templates](https://keats.github.io/tera/docs/) for HTML responses formation,     
[PostgreSQL](https://www.postgresql.org/) as database. 

In general,  I follow the approach from the books: [The Rust Programming Language; Final Project: Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) and    
[Prabhu Eshwarla, Rust Servers, Services, and Apps](https://www.manning.com/books/rust-servers-services-and-apps),      
[Prabhu Eshwarla, Rust Servers, Services, and Apps, GitHub code of the book](https://github.com/peshwar9/rust-servers-services-apps)

TODO: better custom error handling   
TODO: several more views from DB     
TODO: custom views for members     
TODO: possibility to add and view interesting facts about plants     
TODO: possibility for members to make comments  
TODO: dockerize the app   
TODO: the work is in progress 