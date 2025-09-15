# Multithreaded Web Server in Rust

A simple multithreaded web server written in Rust to demonstrate key concepts in networking, HTTP, and concurrency using threads and sockets.

---

## Overview

This project is part of a learning journey to understand how web servers work at a low level. It implements:

✔ A basic TCP listener  
✔ Parsing simple HTTP requests  
✔ Crafting proper HTTP responses  
✔ A thread pool for handling multiple requests concurrently  

This is **not a production-ready server**, but an educational exercise to build a better understanding of Rust’s systems programming capabilities.

---

## What you'll learn

- How TCP works under the hood  
- Parsing HTTP requests manually  
- Writing proper HTTP response headers and bodies  
- How to use threads to handle multiple connections  
- Implementing a thread pool from scratch  
- Why and when to prefer lower-level implementations over external crates

---

## Features

- Accepts TCP connections on a socket  
- Handles GET requests for `/` and `/sleep` endpoints  
- Uses a custom thread pool to improve throughput  
- Clean and minimal design for learning purposes

---

## Requirements

- Rust (latest stable version recommended)
- Cargo for building and running the project

---

## How to run

```bash
git clone https://github.com/RAHULDINDIGALA-32/Multithreaded-Web-Server.git
cd Multithreaded-Web-Server
cargo build --release
cargo run
