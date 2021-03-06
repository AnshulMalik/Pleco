# Pleco

Pleco is a chess Engine inspired by Stockfish, written entirely in Rust.

This project aims to utilize the efficiency of Rust to create a Chess Bot with the speed of modern chess engines.

[![Build Status](https://api.travis-ci.org/sfleischman105/Pleco.svg?branch=master)](https://travis-ci.org/sfleischman105/Pleco)
[![Build Status](https://api.travis-ci.org/sfleischman105/Pleco.svg?branch=Beta-Branch)](https://travis-ci.org/sfleischman105/Pleco)

- [Documentation](https://docs.rs/pleco)

Planned & Implemented features
-------


The internal Board Implementation aims to have the following features upon completion
- [x] Bitboard Representation of Piece Locations:
- [x] Ability for concurrent Board State access, for use by parallel searchers
- [x] Full Move-generation Capabilities
- [x] Statically computed information (including Magic-Bitboards)
- [x] Zobrist Hashing
- [ ] UCI protocol implementation
- [ ] Allowing matches against Human Player



The AI Bot aims to have the following features:
- [x] Alpha-Beta pruning
- [x] Multi-threaded search with rayon.rs
- [x] Queiscience-search
- [x] MVV-LVA sorting
- [x] Iterative Deepening
- [x] Aspiration Windows
- [x] Futility Pruning
- [x] Transposition Tables
- [ ] Null Move Heuristic
- [ ] Killer Moves


  
Contributing
-------

Any and all contributions are welcome! Open up a PR to contribute some improvements. Look at the Issues tab to see what needs some help. 


  
License
-------
Pleco is distributed under the terms of the MIT license. See LICENSE-MIT for details. Opening a pull requests is assumed to signal agreement with these licensing terms.