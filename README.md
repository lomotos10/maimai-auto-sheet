# maimai auto google sheet generator

## Directions

1. Set appropriate `data` html, `intl_del.txt`, `ordering.txt` files

2. `cargo run` to get `charts`

3. Copy a csv file in `charts` to sheet named `Sheet1` of a Google Sheet, create sheet `Sheet2` and `Sheet0` with proper formatting, copy `sheets.gs` to Apps Script and run `generate_sheet` macro.

## TODO

- [x] Left-align/Middle-align based on title length
  - [ ] Currently bugged on non-roman characters
