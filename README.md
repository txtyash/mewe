# MEWE

mewe is a simple cli dictionary app that fetches data from the merriam webster site.
Although I do not have any permmissions from merriam, mewe scrapes very little harmless data.

## USAGE

After cloning the repository, simply pass your word queries to the app & run:
```
❯ cargo run -- stuff
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/mewe stuff`
Query: stuff
Definition: The meaning of STUFF is materials, supplies, or equipment used in various activities.
```
or wrapped in quotes
```
❯ cargo run -- "stuff it"
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/mewe 'stuff it'`
Query: stuff it
Definition: The meaning of STUFF IT is —used as an angry and rude way to say that one does not want something or is not interested in something. Ho
```
handles misspelled words
```
❯ cargo run -- stuffx
Finished dev [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/mewe stuffx`
Query: stuffx
You probably misspelled it.
Suggestions: stuffs,stuff,stubs,stuffy,stuffies,stuffed,stuffiest,stuffer,stubes,stuffie,stuff gown,restuffs,stuff shot,stuff it,stupas,sluff,sluffs,snuff,puffs,puffy,
```
