<a id="readme-top"></a>

<h1>Simple-Cli-News</h1>

### Prerequisites
Just make sure you have cargo installed
```sh
curl https://sh.rustup.rs -sSf | sh
```

### Installation

```sh
cargo install simple-cli-news
```

## Usage

Print top headlines:
```sh 
simple-cli-news
```
Print news including a query:
```sh
simple-cli-news q (query)
```
Print news from a specific source:
```sh 
simple-cli-news source (source id)
```
Get a list of sources from a country:
```sh 
simple-cli-news list sources [Country's 2 digit iso]
```

## Roadmap

- [ ] Use personal api
- [ ] Allow user to set country
- [ ] Who knows what else I'll add