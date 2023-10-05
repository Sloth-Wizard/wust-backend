# Compose our services

Follow these steps
```
cd ./docker/dev/

docker-compose up
```

**If you are setting up the 1st time, you should see that our `wust` service failed and exited with a 101**    
Fear not, it only means our database isn't yet setup inside our docker container, i'll explain it next step.     

If you have no errors, you can visit `127.0.0.1:3000/api/series` in your browser to test.

# Import our actual database to the image running in the container (follow only if wust service didn't start)
In a new terminal, copy the files from our project into the docker container       
First get to the root of our project and run
```
docker cp ./docker/dev/dump.sql wust-db/dump.sql
```

Then we can import that sql by connecting to the docker mysql cli and ```SOURCE``` the file
```
docker exec -it wust-db mysql -h localhost -u root -pwust

MariaDb [(none)] > CREATE DATABASE db;
MariaDb [(none)] > USE db;
MariaDb [db] > SOURCE ./dump.sql;
MariaDb [db] > quit
```

Then we remove the dump.sql file from the container
```
docker exec -it wust-db rm ./dump.sql
```

# Start our wust service (follow only if wust service didn't start)
Send a ctrl_c signal to your initial term to shutdown the running container then follow there steps   

- Delete the container and linked images
- Re-run the ```docker-compose up``` command


# Usefull commands

Rebuild the rust image
```
docker build -t wust .
```

To get the container id or name
```
docker container ls
```

Then we can execute a command
```
docker exec -it <CONTAINER ID or NAMES> <command>
```

For example, to connect to the mariadb service running inside the docker
```
docker exec -it wust-db mysql -h localhost -u root -pwust
```

Inspect the ip of the container   
```
docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' wust-db
```
