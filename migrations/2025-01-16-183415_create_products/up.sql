CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    brand VARCHAR NOT NULL,
    category VARCHAR NOT NULL,
    unit VARCHAR NOT NULL,
    price DECIMAL NOT NULL,
    stock INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ
);
