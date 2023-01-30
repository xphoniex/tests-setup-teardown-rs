# What is this?

This macro allows you to add `setup` and `teardown` for tests. Longs setups distract you and clutters the space on each test, which ideally should be used to focus on the test logic itself.

# Example

Instead of having to do:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat() {
        let db = reserve_path_for_db()
                    .make_tables_for_all_animals()
                    .unwrap();
        let animals = populate_animals_in(&db)
                    .then_add_breeds()
                    .then_listen()
                    .unwrap();

        // now test logic
        assert_eq!(animals.is("sphynx", "cat"), true);
    }

    #[test]
    fn test_dog() {
        let db = reserve_path_for_db()
                    .make_tables_for_all_animals()
                    .unwrap();
        let animals = populate_animals_in(&db)
                    .then_add_breeds()
                    .then_listen()
                    .unwrap();

        // now test logic
        assert_eq!(animals.is("bulldog", "dog"), true);
    }
}
```

You can now do:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tests_setup_teardown::{setup, setup_fn};

    #[setup_fn(visible = animals)]
    fn setup() {
        let db = reserve_path_for_db()
                    .make_tables_for_all_animals()
                    .unwrap();
        let animals = populate_animals_in(&db)
                    .then_add_breeds()
                    .then_listen()
                    .unwrap();
    }

    #[test]
    #[setup(visible = animals)]
    fn test_cat() {
        // now test logic
        assert_eq!(animals.is("sphynx", "cat"), true);
    }

    #[test]
    #[setup(visible = animals)]
    fn test_dog() {
        // now test logic
        assert_eq!(animals.is("bulldog", "dog"), true);
    }
}
```
