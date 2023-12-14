# pg_str: the postgresql extension for strings

add some good default string manipulation functions to postgresql. build using the rust library pgrx: [https://github.com/pgcentralfoundation/pgrx](https://github.com/pgcentralfoundation/pgrx).


function api and behavior is inspired by the laravel web framework: [https://laravel.com/docs/10.x/strings](https://laravel.com/docs/10.x/strings)

## installation
```
git clone https://gitea.publicmatt.com/public/pg_str.git
cd pg_str
cargo pgx package # run cargo install pgx first
sudo make install # adjust Makefile if using different version of postgresql than 13.
```
this puts the binaries and sql into the right folder location. next you need to create the extension in postgresql:

```
psql
> create extension pg_str; # installs functions in a 'public' schema. 
> select str_markdown('# Hello '
|| str.snake('pg str')
|| '- ~~using programming language for str manipulations~~ 
- **do it all in postgresql** ');
```

## api thus far:

- [x] after
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
- [x] length
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
- [x] random
- [] remove
- [x] replace
- [] replaceArray
- [] replaceFirst
- [] replaceLast
- [] reverse
- [x] singular 
- [x] slug 
- [x] snake 
- [x] start
- [] startsWith
- [x] studly 
- [x] substr
- [] substrCount
- [] substrReplace
- [x] title 
- [] ucfirst
- [x] upper 
- [x] uuid
- [] wordCount
- [] words
