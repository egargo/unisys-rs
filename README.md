## University System Backend

This is a Rust implementation of a [University System Backend](https://github.com/lemeow125/UniversitySystem-Backend).

## Dependencies

- [Docker Engine](https://docs.docker.com/engine/)
- [Rust](https://www.rust-lang.org/) (MSRV: 1.70.0)
- [SQLite](https://www.sqlite.org/index.html)
- [clang](https://clang.llvm.org/)
- [mold]()

## Setup

```bash
# Clone the repository with SSH
git clone git@github.com:egargo/UniversitySystem-Backend.git

# or with HTTPS
git clone https://github.com/egargo/UniversitySystem-Backend.git

# Go to the `api` directory
cd UniversitySystem-Backend

# Create a SQLite database
sqlite3 <dbname>

# Setup Config.toml by making a copy then edit the file with `nvim`/`vim`/`vi`/`nano` or with your text editor of choice
cp -v Config.example.toml Config.toml
```

## Run with Docker

```bash
# Build and start the api Docker container in detach mode
docker compose up -d api

# To test that everything is setup correctly run
curl -s http://0.0.0.0:8000 | jq
```
