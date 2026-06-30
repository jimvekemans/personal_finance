# Finance Tracker

A self-contained, highly performant personal finance tracker and lightweight data engineering environment.

## Architecture

* **Database:** PostgreSQL (with ADBC connector compatibility)
* **Backend:** Rust (Axum) using Apache Arrow ADBC drivers
* **Frontend:** Vite, TypeScript, Native CSS
* **Data Engineering:** Python (uv) workspace
* **Orchestration:** Docker Compose

## Prerequisites

* Docker / Docker Compose
* Make

## Installation & Usage

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd finance_tracker
    ```

2.  **Start the cluster:**
    The provided Makefile handles host UID/GID mapping dynamically to prevent volume permission locking across different OS environments.
    ```bash
    make up
    ```

3.  **Access the application:**
    * Frontend: `http://localhost:3000`
    * Backend API: `http://localhost:8080/api`
    * Database: `localhost:5432`

4.  **Stop the cluster:**
    ```bash
    make down
    ```

5.  **Clean up data:**
    Warning: This removes the local postgres volume data.
    ```bash
    make clean
    ```