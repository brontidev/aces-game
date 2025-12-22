# Game for aces hackathon

Board game that's like chess & capture the flag but instead of capturing the
flag, you have to protect your flag from being touched by the other player's
flag

- Each player starts with a set of pieces and a "core piece" (the equivalent of
  the flag in the title)
- The goal is to bring your core piece next to the other player's core piece
- When 2 core pieces are adjecent (including diagonally), they are considered
  "touched" and the round ends, and the player that last moved their core piece
  gets a point
- The first player to get 3 points wins
- The game can be played with at least 2 players, as the pieces can be split up
  and shared between players
- The pieces are like chess pieces but with some differences
  - The monarch is the headmaster, and is the only piece that can move the core
    piece
  - Core piece; Can only be moved when adjacent to the monarch, can move 1-2
    spaces in any direction
  - Left & Right Brutes; Can move 1 spaces in horizontal or vertical direction,
    can forfeit moving to capture a piece in front of them
  - Tank; Can move 1 spaces horizontally or vertically, but if there is a piece
    that is not the core in front of it, it can dash though it, moving 2 pieces
    in that direction, and removing that piece from the board

Name ideas:

- CMBT ("Combat"), stands for the piece names, Core, Monarch, Brute, Tank
- "Core Battle"