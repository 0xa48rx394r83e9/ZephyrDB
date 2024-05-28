# ZephyrDB ✦༺✧

ZephyrDB is a high-performance, in-memory database library implemented in Rust. It provides a simple and efficient way to store, retrieve, and query data using a key-value approach. ZephyrDB is designed to be fast, lightweight, and easy to integrate into Rust projects.

## ✦༺ Features ༻✦

- ✧ High-performance in-memory storage
- ✧ Key-value data model
- ✧ Support for various data types (integers, floats, strings, booleans)
- ✧ Multiple storage backends (B-tree, HashMap)
- ✧ Flexible querying capabilities
- ✧ Modular and extensible design
- ✧ Thread-safe and async-ready
- ✧ Persistence and backup options

## ✦༺ Table of Contents ༻✦

- [Installation](#-installation)
- [Usage](#-usage)
- [Storage Backends](#-storage-backends)
- [Querying](#-querying)
- [Persistence](#-persistence)
- [Performance](#-performance)
- [Roadmap](#-roadmap)
- [License](#-license)
- [Resources](#-resources)

## ✦༺ Installation ༻✦

To use ZephyrDB in your Rust project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
zephyrdb = "0.1.0"
```

## ✦༺ Usage ༻✦

Here's a basic example of how to use ZephyrDB:

```rust
use zephyrdb::Database;
use zephyrdb::types::Value;

fn main() {
    // Create a new database instance
    let mut db = Database::new("btree");

    // Insert key-value pairs
    db.insert("name", Value::String("John Doe".to_string()));
    db.insert("age", Value::Int(30));

    // Retrieve values by key
    let name = db.get("name");
    let age = db.get("age");

    println!("Name: {:?}", name);
    println!("Age: {:?}", age);
}
```

For more detailed usage examples and API documentation, refer to the [API docs](https://docs.rs/zephyrdb).

## ✦༺ Storage Backends ༻✦

ZephyrDB supports multiple storage backends to accommodate different use cases and performance requirements:

- **B-tree** (`"btree"`): A balanced tree structure that provides efficient key-value storage and retrieval. It offers good performance for range queries and maintains keys in sorted order.
- **HashMap** (`"hashmap"`): A hash table implementation that provides fast key-value lookups and insertions. It offers excellent performance for point queries but does not maintain key ordering.

To specify the desired storage backend, pass the corresponding string identifier (`"btree"` or `"hashmap"`) when creating a new `Database` instance.

## ✦༺ Querying ༻✦

ZephyrDB provides a flexible querying mechanism to retrieve data based on specific conditions. You can create a `Query` object, add conditions, and execute the query on the database.

```rust
use zephyrdb::Database;
use zephyrdb::query::{Query, QueryCondition};
use zephyrdb::types::Value;

fn main() {
    let db = Database::new("btree");

    // Insert data
    // ...

    // Create a query
    let mut query = Query::new();
    query.add_condition(QueryCondition::Equals("age".to_string(), Value::Int(30)));

    // Execute the query
    let results = db.execute(&query);

    println!("Query Results: {:?}", results);
}
```

For more advanced querying capabilities and examples, refer to the [Querying guide](https://github.com/0xa48rx394r83e9/ZephyrDB/docs/querying.md).

## ✦༺ Persistence ༻✦

ZephyrDB provides options for data persistence and backup to ensure data durability. You can save the database state to disk and load it back into memory when needed.

```rust
use zephyrdb::Database;

fn main() {
    let mut db = Database::new("btree");

    // Insert data
    // ...

    // Save the database to a file
    db.save_to_file("backup.db").expect("Failed to save database");

    // Load the database from a file
    let restored_db = Database::load_from_file("backup.db").expect("Failed to load database");
}
```

For more information on persistence and backup strategies, refer to the [Persistence guide](https://github.com/0xa48rx394r83e9/ZephyrDB/docs/persistence.md).

## ✦༺ Performance ༻✦

ZephyrDB is designed with performance in mind. It leverages Rust's ownership system, memory safety, and concurrency primitives to deliver high-speed data storage and retrieval. The choice of storage backend (`"btree"` or `"hashmap"`) allows you to optimize for specific workloads and access patterns.

For benchmarks and performance comparisons, refer to the [Performance guide](https://github.com/0xa48rx394r83e9/ZephyrDB/docs/performance.md).

## ✦༺ Roadmap ༻✦

Here are some planned features and improvements for future releases of ZephyrDB:

- [ ] Support for additional data types (arrays, dictionaries, etc.)
- [ ] Transactions and ACID properties
- [ ] Disk-based storage option for larger datasets
- [ ] Replication and distributed storage
- [ ] Integration with popular Rust web frameworks
- [ ] Improved query optimization and indexing strategies

If you have any suggestions or feature requests, please open an issue on the [GitHub repository](https://github.com/0xa48rx394r83e9/ZephyrDB/issues).

## ✦༺ License ༻✦

ZephyrDB is released under the [MIT License](https://github.com/0xa48rx394r83e9/ZephyrDB/blob/main/LICENSE).

## ✦༺ Resources ༻✦

- [Rust Programming Language](https://www.rust-lang.org/)
- [B-tree Data Structure](https://en.wikipedia.org/wiki/B-tree)
- [HashMap Data Structure](https://en.wikipedia.org/wiki/Hash_table)
- [Key-Value Database](https://en.wikipedia.org/wiki/Key-value_database)
- [In-Memory Database](https://en.wikipedia.org/wiki/In-memory_database)
