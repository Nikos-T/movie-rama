# MovieRama

Run `docker-compose up` to start the web-server.

The application will be available on localhost:8080,
and the db on postgres://db_admin_user:password123@localhost/movierama

The authentication is done using jwt-tokens which are stored in the sessionStorage
(giving the ability to have different users on different browser tabs).

