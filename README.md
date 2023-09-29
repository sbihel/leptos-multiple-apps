# Leptos Demo for Serving Multiple Frontends

## Usage

First, build the first frontend app:
```
cargo leptos build --project app1
```

Then, build and serve the second:
```
cargo watch --project app2
```

You can now access them on localhost:3000 and localhost:30001 respectively.
