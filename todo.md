### Project Helpers
- add cargo watch [X]
- add mold for dev builds [X]
- create break-test (awaiting better engine modulation)
    - module and data driven loader
- switch to node to serve solution
- add more make commands

### Engine
- bundle bind group with texture and pipeline
- separate pipeline and pass
- create scene
- load and render font
- load and play sound

### Game
- get more logic going
    - Determine eventing solution with pieces and game controller/board
- create nginx docker container hosting wasm version
- create server
- create game controller
- create render controller
    - pixel perfect grid solution
- create exe icon for windows/linux

- 2v2 setup

- ios
- android

### Render
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
