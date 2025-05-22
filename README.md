
# Dodo Payments Backend

A simple backend API service for managing users and transactions.

---

## 🚀 Quickstart Guide (Docker-Based)

### 1. Clone the Repository

```bash
git clone https://github.com/gourav-banerjee7/dodo_payments_backend.git
cd dodo_payments_backend
````

---

### 2. Set Up Environment

Create a `.env` file in the root directory:

```env
DATABASE_URL=postgres://postgres:password@localhost/dodo
JWT_SECRET=supersecretkey
HOST=0.0.0.0
PORT=3000
```

> 🔑 You can change the database name, user, password, or port as needed.

---

### 3. Run Docker Compose

```bash
docker compose up -d --build
```

This will:

* Start a PostgreSQL database on port `5432`
* Build and run the Rust backend app on port `3000`

---

### 4. Run Database Migrations

Open a new terminal and run:

```bash
docker exec -it dodo_payments_backend-app-1 bash
cargo sqlx migrate run
```

> If the above command fails, ensure the `.env` is loaded inside the container.

---

### 5. Access the API

* Base URL: `http://localhost:3000`
* Example endpoint: `GET /balance`

Use Postman or `curl` to test endpoints.

---

## 🧪 Run Tests

### 🐳 Run Test Database

```bash
docker compose -f docker-compose.test.yaml up -d
```

### 🧪 Run Unit/Integration Tests

Make sure test `.env` is configured, then:

```bash
cargo test
```

---

## 🧹 Cleanup

Stop and remove all containers:

```bash
docker compose down
```

---

## 📁 Folder Structure

```
.
├── src/
│   ├── handlers/         # Request handlers
│   ├── models.rs         # DB models and schemas
│   ├── routes.rs         # API routes
│   ├── middleware/       # Auth middleware
│   ├── db.rs             # DB connection logic
│   ├── lib.rs            # App logic
│   └── main.rs           # Entry point
├── tests/                # Integration and unit tests
├── migrations/           # SQLx migration files
├── Dockerfile
├── docker-compose.yaml
├── docker-compose.test.yaml
├── .env
└── README.md
```

---