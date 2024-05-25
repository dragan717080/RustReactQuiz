# Quiz App

## Quiz App made with Rust (Rocket), PostgreSQL, React, TypeScript and Tailwind

https://github.com/dragan717080/RustReactQuiz/assets/135660124/bd969a7e-6b79-4f92-82eb-6063cc140e70

## Technologies Used

- **Rust (Rocket)**

Rust is a systems programming language and it is the fastest language in the world. It is known for its performance and safety. Rocket is a web framework for Rust that makes it easy to build web applications.

- **PostgreSQL**

PostgreSQL is a RDBMS known for its reliability, robust feature set, and extensibility. It is highly regarded for its ability to handle complex queries, manage high concurrency, and provide advanced data types and indexing capabilities. 

- **React**

React is a popular JavaScript library for building user interfaces, particularly for web applications. It allows developers to create interactive and dynamic UI components, making it easier to build complex and responsive user interfaces. follows a component-based architecture, where UIs are made into reusable components.

- **TypeScript**

TypeScript is a strongly typed superset of JavaScript that enhances code maintainability and scalability. It allows us to catch errors during development and provides better tooling support, leading to more robust applications.

- **Tailwind CSS**

Tailwind CSS is a utility-first CSS framework that enables rapid UI development. Its utility classes make it easy to create responsive and custom-designed user interfaces without writing custom CSS.

## How to install

You need Rust and Cargo (Rust dependency manager) installed on your machine.

After cloning this git repository:

1. Create `.env` file in the root of the `/api` directory and in it write your PostgreSQL connection string.

`DB_CONNECTION_STRING=YOUR_CONNECTION_STRING`

2. Create `.env` file in the root of the `/frontend` directory and in it write your API base url.
`VITE_API_BASE_URL=http://127.0.0.1:8000`

2. Run the backend
```
cd api
rustup override set nightly
cargo install
cargo run
```

3. Run the frontend
```
cd frontend
npm install
npm run dev
```
