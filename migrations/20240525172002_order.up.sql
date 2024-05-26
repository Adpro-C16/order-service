-- Add up migration script here
-- Add up migration script here
CREATE TABLE IF NOT EXISTS "orders" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    supermarket_id INTEGER NOT NULL,
    product_name VARCHAR(255) NOT NULL,
    quantity INTEGER DEFAULT 1,
    subtotal INTEGER NOT NULL,
    status VARCHAR(255) NOT NULL DEFAULT 'Waiting Payment',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);