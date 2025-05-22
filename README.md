
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

## 📚 API Documentation

### 🔗 Base URL

```
http://localhost:3000
```

### 📖 Endpoints

#### POST `/register`

Register a new user.

**Request**

```json
{
  "email": "user@example.com",
  "password": "your_password"
}
```

**Response**

```json
{
  "id": "uuid",
  "email": "user@example.com",
  "created_at": "timestamp"
}
```

---

#### POST `/login`

Log in a user and receive a JWT.

**Request**

```json
{
  "email": "user@example.com",
  "password": "your_password"
}
```

**Response**

```json
{
  "token": "jwt_token"
}
```

---

#### POST `/transaction`

Create a new transaction.

**Headers**

```
Authorization: Bearer <token>
```

**Request**

```json
{
  "amount": 100,
  "description": "test deposit"
}
```

**Response**

```json
{
  "id": "uuid",
  "user_id": "uuid",
  "amount": 100,
  "description": "test deposit",
  "created_at": "timestamp"
}
```

---

#### GET `/balance`

Get total balance for the authenticated user.

**Headers**

```
Authorization: Bearer <token>
```

**Response**

```json
{
  "balance": 500
}
```

---

## 🧪 Run Tests

### 1. Start Test Database

```bash
docker compose -f docker-compose.test.yaml up -d
````

This will spin up a PostgreSQL test database on port `5433`.

---

### 2. Run Migrations on Test DB

```bash
sqlx migrate run --database-url postgres://postgres:password@localhost:5433/dodo_test
```

Ensure this command is run **from the host machine** and that your `migrations/` folder exists.

---

### 3. Run Unit and Integration Tests

```bash
cargo test
```

Make sure your `.env` file points to the test database (`dodo_test`) before running this, or override via:

```bash
DATABASE_URL=postgres://postgres:password@localhost:5433/dodo_test cargo test
```

---

## 🎥 Demo Video

📽️ [Watch the Demo Video](https://screenrec.com/share/Znyc6poW2X)

Includes:

* User registration and login
* Creating and listing transactions
* Auth-based access control
* Design overview and architecture choices

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

## 🛠 Tech Stack

* **Rust**
* **Axum** (Web framework)
* **SQLx** (Async PostgreSQL ORM)
* **Docker + Docker Compose**
* **JWT for Authentication**
* **PostgreSQL**

---