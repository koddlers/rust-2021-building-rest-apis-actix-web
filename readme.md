# Building REST APIs in Rust 2021 with Actix Web

by Edward Curren

Given how mature and popular HTTP is as a protocol, web APIs are a popular solution for client and server interaction.
This course will teach you how to implement secure, reliable, and blazingly fast web APIs using Rust.

## Course Objective

Applications need to serve data to systems anywhere in the world. Web APIs are built on the HTTP protocol which is not
only a mature and ubiquitous protocol for data exchange, but also doesn't care what language or technology you use to
send data over it. In this course, Building REST APIs in Rust 2021 with Actix Web, you’ll learn to build secure,
reliable, and blazingly fast RESTful APIs using Rust. First, you’ll explore setting up a standalone HTTP server built
into your API project. Next, you’ll discover how to set up secure API endpoints that your API will serve. Finally,
you’ll learn how to integrate a web front end to use your Rust API. When you’re finished with this course, you’ll have
the skills and knowledge needed to implement secure, reliable, and blazingly fast web APIs using Rust.

### Course Content

```markdown
├── 01 - Course Overview
│ └── 01 - Course Overview
├── 02 - Getting Started
│ ├── 01 - Overview
│ ├── 02 - Project Description
│ ├── 03 - Development Environment Setup
│ ├── 04 - Project Demo
│ └── 05 - Summary
├── 03 - Persisting Data
│ ├── 01 - Overview
│ ├── 02 - Schemas
│ ├── 03 - Database Setup
│ ├── 04 - Database Function Definitions
│ ├── 05 - Database Function Code
│ └── 06 - Summary
├── 04 - Endpoints
│ ├── 01 - Overview
│ ├── 02 - Endpoint Handlers
│ ├── 03 - Writing the Endpoint Code
│ ├── 04 - The API Server
│ ├── 05 - Testing the API
│ └── 06 - Summary
├── 05 - Authentication and Security
│ ├── 01 - Overview
│ ├── 02 - Adding a User
│ ├── 03 - Server Configuration
│ ├── 04 - Authentication
│ └── 05 - Summary
└── 06 - Front End
├── 01 - Overview
├── 02 - Getting Started with the Code
├── 03 - Finishing the Code
└── 04 - Summary
```

### Database Setup

Change directory into the `assets` directory. Then, run the following command,

```shell
sqlite3.exe duckair.db ".read database.sql"
```

**Note -** Make sure you have `sqlite3` set up before running the above command.

### Resources

* Course Link - <https://www.pluralsight.com/courses/rust-2021-building-rest-apis-actix-web>
* Course Repository - <https://github.com/sentrid/ps-rust-actix-apihttps://github.com/sentrid/ps-rust-actix-api>