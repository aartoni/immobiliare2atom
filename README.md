# immobiliare2atom

`immobiliare2atom` generates Atom feeds from Immobiliare.it searches. The project is based on [`ebay2atom`](https://github.com/aartoni/ebay2atom), a Rust script that generates an Atom feed from eBay searches.

## Installation

Clone, the project, customize the script as needed, then install as follows.

```sh
cargo install --path .
```

## Usage

The program can be directly invoked from the Newsboat `urls` configuration file.

    # Example query
    "exec:immobiliare2atom \"https://www.immobiliare.it/api-next/search-list/real-estates/?fkRegione=xyz&idProvincia=XY&idComune=1234&idMZona[0]=123&idMZona[1]=321&idQuartiere[0]=1234&idNazione=IT&idContratto=1&idCategoria=1&prezzoMassimo=125000&superficieMinima=60&localiMinimo=3&criterio=dataModifica&ordine=desc&__lang=it&path=/vendita-case/monopoli/\"" "~Case in vendita a Monopoli"
