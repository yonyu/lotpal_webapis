# LotPal Application Backend

```
cargo new lotpal_webapis

git branch -M master
git remote add origin https://github.com/yonyu/lotpal_webapis.git
git push -u origin master

cargo add actix-web@4

cargo add serde
```

# API endpoints

Get all records
HTTP GET
http://127.0.0.1:8080/cn649

Get a record
HTTP GET
http://127.0.0.1:8080/cn649/{id}

Create a new record
HTTP POST
http://127.0.0.1:8080/cn649

Update a record
HTTP PUT            
http://127.0.0.1:8080/cn649/{id}

Delete a record
HTTP DELETE
http://127.0.0.1:8080/cn649/{id}
            

# Swagger and OpenAPI support

## API documentation will be available at:

Swagger UI: http://localhost:8080/swagger-ui/

OpenAPI JSON: http://localhost:8080/api-docs/openapi.json

## The Swagger UI will provide:

Interactive documentation for all endpoints

Request/response examples

Try-it-out functionality

Schema definitions

API descriptions

## Features included:

Full OpenAPI 3.0 specification

Detailed request/response documentation

Path and query parameter documentation

Request body schemas

Response schemas

Error responses

Example values

This implementation provides a professional, interactive API documentation that will help developers understand and use the API effectively.

