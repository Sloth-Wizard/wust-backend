# WUST-API

Rewrite of the WebtoonFactory application backend    

## Setup

Clone 
```
git clone git@github.com:AlexisADupuis/wust-api.git
```

Move into clone folder
```
cd wust-api
```

First we check dependencies so run
```
cargo check
```

If you get errors, verify the ```Cargo.toml``` file and update dependencies then re-run the check    

If no problem are encoutered, start the server
```
cargo run
```
This will run the server on the ```127.0.0.1:3000``` by default, you can change it to what you want    

## Database

Be sure to have a clone of the ```dev``` or ```prod``` database up and running locally
