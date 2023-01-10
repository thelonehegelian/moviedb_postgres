# Instruction for setting up a docker postgres db

- create yaml file with settings: 


  ```json
  version: "3.9"
services:
  postgres:
    image: postgres:14
    environment:
      - POSTGRES_USER=nobody
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=library
    ports:
      - "5243:5432"
  ```

- run command `docker-compose up`
- run `docker exec -it <containername> bash`
- run `psql -d postgres -U <POSTGRES_USER>` or `psql -U <POSTGRES_USER>`

