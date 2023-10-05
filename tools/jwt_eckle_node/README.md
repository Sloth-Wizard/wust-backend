# JWT Generator

If you are running this from a windows machine, you're probably using Powershell  
You need to run the following steps before executing the bash script

First, make sure you are able to use wsl    
Then run this command
```
npm install pem-jwk
```
Then go into the wsl bash
```
bash
```
Then update your wsl system so you can install the `jq` tool
```
sudo apt update

sudo apt install jq
```

Then to run our sh script
```
bash

./create_jwt.sh
```

If you have git bash or another bash like terminal, you can use it instead of all the previous steps and simply call
```
./create_jwt.sh
```
