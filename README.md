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

```bash
docker compose up -d --build
```