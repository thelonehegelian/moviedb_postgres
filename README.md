TODO: 
---

- Fix insert query (creation query ran successfully)
- expand db for dynamic tasks
- try diesel crate: https://diesel.rs/guides/getting-started.html

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

