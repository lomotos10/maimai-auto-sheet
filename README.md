# maimai auto google sheet generator

## Directions

1. Set appropriate `data/(intl_del, intl_url, jp_url).txt` files (check version is correct)

2. `cargo run` to get `charts`

3. Copy a csv file in `charts` to sheet named `Sheet1` of a Google Sheet, create sheet `Sheet2` and `Sheet0` with proper formatting, copy `sheets.gs` to Apps Script and run `generate_sheet` macro.

## TODO

- [x] Left-align/Middle-align based on title length
  - [ ] Currently bugged on non-roman characters
- [ ] Find online source for deleted songs - ditch `intl_del.txt`
  - Use 'version' tag?
- [ ] `intl_url` json is outdated compared to live version - any way to add updates?
- [ ] Auto-create Google Sheet
