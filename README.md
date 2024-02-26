# meal_website

## Design

```bash
meal_website/
│
├── backend/                  # Backend code (Rust, etc.)
│   ├── src/
│   └── Cargo.toml
│
├── frontend/                 # Frontend code (HTML, CSS, JavaScript)
│   ├── index.html
│   └── style.css
│
├── db/                       # Database scripts and configurations
│   ├── schema.sql            # SQL script to create database schema
│   └── migrations/           # Database migration files (if using a migration tool)
│
├── auth
│
├── Dockerfile                # Dockerfile for containerization
├── docker-compose.yml        # Docker Compose file for multi-container setup
└── README.md                 # Project documentation
```

### Quickstart

### Backend Quickstart

Setup environment file by renaming `meals-example.env` -> `.env`:

```bash
# FILE: meal.env
POSTGRES_USER=mealsuser
POSTGRES_PASSWORD=mealspassword
POSTGRES_DB=mealsdb
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
DATABASE_URL=postgresql://mealsuser:mealspassword@localhost:5432/mealsdb
```

```bash
docker compose up -d --build
```