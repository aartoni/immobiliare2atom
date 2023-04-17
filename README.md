# ebay2atom

`ebay2atom` is a [Newsboat](https://newsboat.org/) filter script to generate Atom feeds from eBay searches. The project is based on [`ebay2rss`](https://github.com/almog/newsboat-ebay2rss-filter), a Python script that generates an RSS feed from eBay searches, many thanks to the original author.

## Compilation

Customize the script as needed, then compile in release mode as follows.

```sh
cargo build --release
```

The executable can be found in `./target/release/ebay2atom`

## Usage

The filter script can be directly invoked from the Newsboat `urls` configuration file.

    # Example query
    filter:~/.local/bin/ebay2atom:https://www.ebay.com/sch/i.html?_nkw=cool+gadget "Gadgets" "~eBay"

The above query would produce the following output.

```xml
<?xml version="1.0"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>cool gadget</title>
  <id></id>
  <updated>2053-04-11T20:54:49.406509025+00:00</updated>
  <generator uri="https://github.com/aartoni/ebay2atom/" version="0.1.0">ebay2atom</generator>
  <link href="https://www.ebay.com/sch/i.html?_nkw=cool+gadget" rel="alternate" type="text/html"/>
  <entry>
    <title>15 in 1 Survival Kit</title>
    <id>01234567891</id>
    <updated>2053-04-11T20:54:49.406509025+00:00</updated>
    <link href="https://www.ebay.com/itm/01234567891" rel="alternate" type="text/html"/>
    <content type="xhtml"><div xmlns="http://www.w3.org/1999/xhtml"><p>Price: $33.00</p><p>Condition: Brand New</p></div></content>
  </entry>
  <!-- ... -->
  <entry>
    <title>USB Flexible Mini Fan</title>
    <id>01234567890</id>
    <updated>2053-04-11T20:54:49.406509025+00:00</updated>
    <link href="https://www.ebay.com/itm/01234567890" rel="alternate" type="text/html"/>
    <content type="xhtml"><div xmlns="http://www.w3.org/1999/xhtml"><p>Price: $6.92</p><p>Condition: Brand New</p></div></content>
  </entry>
</feed>
```
