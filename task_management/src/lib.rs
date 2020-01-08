mod domain;

#[cfg(test)]
mod tests {

    use crate::domain::*;
    use chrono::*;

    #[test]
    fn test_create_new_task() {
        let expected_name = "study".to_string();
        let expected_time_limit = Local::today();
        match Task::new(&expected_name, &expected_time_limit) {
            Ok(task) => {
                assert_eq!(expected_name, task.name);
                assert_eq!(expected_time_limit, task.time_limit);
            }
            Err(err) => {
                panic!();
            }
        }
    }

    #[test]
    fn test_create_task_invalid_timelimit() {
        let expected_name = "study".to_string();
        let expected_time_limit = Local::today() - Duration::days(1);
        match Task::new(&expected_name, &expected_time_limit) {
            Ok(task) => panic!("{}", task.time_limit),
            Err(err) => assert_eq!("create task error.", err),
        }
    }
}
