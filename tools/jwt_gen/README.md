# JWT Rust generator

All you have to do is be sure you have the latest rust version installed and then you can simply run
```
./jwtgen.sh
```
You will find you generated token inside a newly created `jwt.txt` file.     
To get the pem generated keys, go into the `/src/pem` folder and use the public key to decode the new `JWT`    
Typically you want to include the file as bytes in your validation script    
