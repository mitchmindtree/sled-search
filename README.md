# sled-search [![Build Status](https://travis-ci.org/mitchmindtree/sled-search.svg?branch=master)](https://travis-ci.org/mitchmindtree/sled-search) [![Crates.io](https://img.shields.io/crates/v/sled-search.svg)](https://crates.io/crates/sled-search) [![Crates.io](https://img.shields.io/crates/l/sled-search.svg)](https://github.com/mitchmindtree/sled-search/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/sled-search/badge.svg)](https://docs.rs/sled-search/)

Utility functions for searching a [`sled::Tree`](https://docs.rs/sled/latest).

Includes:

- `search` - allows for searching the `sled::Tree` key space using a guiding
  function.
- `max` - finds the greatest entry in the `sled::Tree` (uses `search`
  internally).
- `pred` - finds the greatest key preceding the given key (uses `search`
  internally).
- `pred_incl` - a version of the `pred` fn that is inclusive of the given key.
