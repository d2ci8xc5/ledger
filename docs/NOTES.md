# notes
### why rust?
- because I only feel half confident using it. I wanted to discover more of common crates and standard library. This only ended up being a half mistake as I had to learn a few new libraries but it was more fun.

### design decisions
- I used a object serialization library to write ledger state to disk. I figured that since I do not have a client/server model it isnt too much less efficient to just write the object to disk. Because I do not have so send packets across a network that means that I do not have to chunk my states into easily accessable pieces (what a database is good for as you wouldn't have to send the whole ledger state across the network, just the change in state).
- I included account and transaction IDs as names are not valid UUID.
### what would I do if I had more time?
- I would probably make this project using a more mature language like C++ or Java. Rust doesn't have a stable (or even functionally complete) GUI library which made things difficult for me.
- I would change the structure of the project to the client & server model.
- I would use a proper database instead of serializing & deserializing objects to disk.
- If I would continue using rust I would make a webUI interface using either the library Rocket or rusts webassembly bindings
### overall
Good exercise. Got me thinking about how the bitcoin ledger system works again and what parts of it that I wanted to use.
