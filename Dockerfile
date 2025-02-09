# Gunakan image Rust yang lebih ramping
FROM rust:1.82.0-slim

# Set directory kerja di dalam container
WORKDIR /app

# Salin semua file ke dalam container
COPY . .

# Perbarui sistem dan install dependensi untuk PostgreSQL
RUN apt-get update && \
  apt-get upgrade -y -o DPkg::Options::=--force-confold && \
  apt-get install -y -o DPkg::Options::=--force-confold \
  curl unzip build-essential pkg-config libssl-dev \
  postgresql-client libpq-dev

# Install cargo-watch untuk pemantauan otomatis
RUN cargo install cargo-watch

# Install sea-orm-cli untuk menjalankan migrasi
RUN cargo install sea-orm-cli

# Build aplikasi dalam mode release
RUN cargo build --release

# Expose port yang akan digunakan oleh aplikasi
EXPOSE 8080

# Perintah untuk menjalankan aplikasi, bisa juga tambahkan migrasi SeaORM sebelum aplikasi berjalan
CMD sea-orm-cli migrate up && ./target/release/my_rust_app
