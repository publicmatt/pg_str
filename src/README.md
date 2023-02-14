# Postgresql String Extension

A better way of handling string manipulation and transformations in Postgresql.

Function api and behavior is inspired by those available in the Laravel web framework: https://laravel.com/docs/8.x/helpers#strings-method-list

## Installation
```
git clone git@github.com:publicmatt/pg_str
cd pg_str
cargo pgx package # run cargo install pgx first
sudo make install # adjust Makefile if using different version of postgresql than 15.
```
This puts the binaries and sql into the right folder location. Next you need to create the extension in postgresql:

```
psql
> create extension pg_str; # installs functions in a schema named 'str'
> select str.markdown('# Hello '
|| str.snake('pg str')
|| '- ~~using programming language for str manipulations~~ 
- **do it all in postgresql** ');
```

## API
- [] after
- [] afterLast
- [x] ascii
- [] before
- [] beforeLast
- [] between
- [x] camel 
- [x] contains
- [x] containsAll
- [] endsWith
- [] finish
- [] headline
- [] is
- [x] isAscii
- [] isUuid
- [x] kebab 
- [] length
- [] limit
- [x] lower 
- [x] markdown 
- [] mask
- [] orderedUuid
- [] padBoth
- [] padLeft
- [] padRight
- [x] plural 
- [] pluralStudly
- [] random
- [] remove
- [x] replace
- [] replaceArray
- [] replaceFirst
- [] replaceLast
- [] reverse
- [x] singular 
- [x] slug 
- [x] snake 
- [] start
- [] startsWith
- [x] studly 
- [] substr
- [] substrCount
- [] substrReplace
- [x] title 
- [] ucfirst
- [x] upper 
- [] uuid
- [] wordCount
- [] words
