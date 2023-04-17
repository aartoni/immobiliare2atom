# immobiliare2atom

`immobiliare2atom` is a [Newsboat](https://newsboat.org/) filter script to generate Atom feeds from Immobiliare.it searches. The project is based on [`ebay2atom`](https://github.com/aartoni/ebay2atom), a Rust script that generates an Atom feed from eBay searches.

## Compilation

Customize the script as needed, then compile in release mode as follows.

```sh
cargo build --release
```

The executable can be found in `./target/release/immobiliare2atom`

## Usage

The filter script can be directly invoked from the Newsboat `urls` configuration file.

    # Example query
    filter:~/.local/bin/immobiliare2atom:https://www.immobiliare.it/affitto-case/roma/?criterio=dataModifica&ordine=desc "~Immobiliare.it (affitto a Roma)"
