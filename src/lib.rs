#[cfg(test)]
mod tests_1 {
    use tests_setup_teardown::{setup, setup_fn};

    #[setup_fn(visible = name)]
    fn setup() {
        let name = "john doe".to_string();
    }

    #[test]
    #[setup(visible = name)]
    fn it_works() {
        assert_eq!(name, "john doe");
    }
}

#[cfg(test)]
mod tests_2 {
    use tests_setup_teardown::{setup, setup_fn};

    #[setup_fn(visible = name, age)]
    fn setup() {
        let name = "john doe".to_string();
        let age = 20;
    }

    #[test]
    #[setup(visible = name, age)]
    fn it_works() {
        assert_eq!(name, "john doe");
        assert_eq!(age, 20);
    }
}

#[cfg(test)]
mod tests_3 {
    use tests_setup_teardown::{setup, setup_fn};

    #[setup_fn(visible = name, location)]
    fn setup() {
        let name = "john doe".to_string();
        let _age = 20;
        let location = "north pole";
    }

    #[test]
    #[setup(visible = name, location)]
    fn it_works() {
        assert_eq!(name, "john doe");
        assert_eq!(location, "north pole");
    }
}
