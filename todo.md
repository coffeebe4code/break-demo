### Project Helpers
- create break-test (awaiting better engine modulation)
    - module and data driven loader
- switch to node to serve solution
- add more make commands
- load textures via wasm rather than file::open in wasm

### Engine
- load and play sound - engine based
- load and render font - engine based

### Game
- get more logic going
    - Determine eventing solution with pieces and game controller/board
- create game controller
- create render controller
    - pixel perfect grid solution
- create exe icon for windows/linux
- create a sound
- create intro scene

- 2v2 board setup

- ios
- android

### Render
- load and render font
- create first shader
    - spinning pieces
    - electric pieces
    - board power use on borders
- media query like sizes for the board.
    - lt 2048, 1024, 512
- create more assets
    - 100, 200, 300, 400 powers
- first load is odd
    - look to load a black screen as fast as possible
    - maybe don't configure any assets or passes/pipelines until after "update" is called

### Server & Client
- create the server.
    - use websockets
- create the client.
- create nginx docker container hosting wasm version and server

### Business
- create Puzzle Battle assets

