# meal_website

## Design

```bash
meal_website/
├── README.md  # Project documentation and setup instructions
├── auth  # Authentication module, possibly for user management
├── backend
│   ├── Dockerfile  # Docker configuration for the backend service
│   └── src
│       ├── lib.rs  # Shared library code
│       ├── main.rs  # Entry point for the backend application
│       └── schema.rs  # Database schema definitions
├── database
│   ├── Dockerfile  # Docker configuration for the database service
│   ├── migrations  # Database migration scripts for schema updates
│   └── schema.sql  # Initial database schema setup
├── docker-compose.yaml  # Defines and runs multi-container Docker applications
├── frontend
│   ├── Dockerfile  # Docker configuration for the frontend service
│   ├── config
│   │   └── custom_nginx.conf  # Custom NGINX configuration for the frontend
│   ├── index.html  # Main entry point for the web application
│   └── script.js  # JavaScript code for frontend logic
└── meals-example.env  # Example environment file, to be renamed to `.env` for use
```

### Quickstart

### Backend Quickstart

Setup environment file by renaming `meals-example.env` --> `meals.env`:

```bash
# FILE: meal.env
POSTGRES_USER=mealsuser # UPDATE
POSTGRES_PASSWORD=mealspassword # UPDATE
POSTGRES_DB=mealsdb # UPDATE
POSTGRES_HOST=database
POSTGRES_PORT=5432
```

```bash
docker compose up -d --build
# homepage at: http://localhost:8080/
# meals data from api at http://localhost:8080/api/meals
```