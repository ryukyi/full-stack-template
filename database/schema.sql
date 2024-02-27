--- Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Meals table
CREATE TABLE meals (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL CHECK (price > 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Orders table
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    meal_id INTEGER REFERENCES meals(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    total_price DECIMAL(10, 2) NOT NULL CHECK (total_price > 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Insert dummy data into Users
INSERT INTO users (username, email, password_hash) VALUES
('john_doe', 'john.doe@example.com', 'hash1'),
('jane_doe', 'jane.doe@example.com', 'hash2');

-- Insert dummy data into Meals
INSERT INTO meals (name, description, price) VALUES
('Spaghetti Bolognese', 'Classic spaghetti with meat sauce', 12.50),
('Vegetarian Pizza', 'Pizza with assorted vegetables', 10.00);

-- Insert dummy data into Orders
INSERT INTO orders (user_id, meal_id, quantity, total_price) VALUES
(1, 1, 2, 25.00),
(2, 2, 1, 10.00);
