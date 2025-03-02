# Blog Platform API  

A high-performance, Rust-based blogging platform API built using **Axum, SQLx, and Tokio** with **PostgreSQL** as the database.  

## 🚀 Features  

- Create, read, update, delete (CRUD) blog posts  
- Search blog posts by title, content, or tags  
- Fast and scalable with **Axum**  
- Asynchronous database operations with **SQLx**  
- Structured logging with **Tower**  

## 🛠️ Tech Stack  

- **Language**: Rust 🦀  
- **Frameworks & Libraries**: Axum, SQLx, Tokio  
- **Database**: PostgreSQL  

## 📌 API Endpoints  

| Method | Endpoint             | Description                      |
|--------|----------------------|----------------------------------|
| `GET`  | `/`                  | Welcome message                 |
| `POST` | `/posts`             | Create a new blog post          |
| `GET`  | `/posts`             | Retrieve all blog posts         |
| `GET`  | `/posts/search?term=` | Search blog posts by a keyword  |
| `GET`  | `/posts/{id}`        | Retrieve a blog post by ID      |
| `PUT`  | `/posts/{id}`        | Update a blog post by ID        |
| `DELETE` | `/posts/{id}`     | Delete a blog post by ID        |

## 🏗️ Setup  

### 1️⃣ Prerequisites  

- Install **Rust**: [rustup.rs](https://rustup.rs/)  
- Install **PostgreSQL**: [postgresql.org](https://www.postgresql.org/download/)  

### 2️⃣ Clone the Repository  

```sh
git clone https://github.com/Abhishek2010DevSingh/Blog-Platform-API
cd blog-platform-api
```

### 3️⃣ Configure Environment  

Create a `.env` file and set up the database URL:  

```
DATABASE_URL=postgres://user:password@localhost/blog_db
```

### 4️⃣ Run Migrations  

```sh
sqlx database create
sqlx migrate run
```

### 5️⃣ Start the Server  

```sh
cargo run
```

The API will be available at `http://localhost:3000`.  

## 📖 Inspiration  

This project is inspired by the **[Blogging Platform API roadmap](https://roadmap.sh/projects/blogging-platform-api)**.  

---

This README keeps it clean, informative, and professional. Let me know if you want modifications! 🚀
