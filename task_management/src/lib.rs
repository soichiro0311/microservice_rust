mod domain;

#[cfg(test)]
mod tests {

    use crate::domain::*;
    use chrono::*;

    #[test]
    fn test_create_new_task() {
        let expected_name = "study".to_string();
        match Task::new(&expected_name, &create_date_str(Local::today())) {
            Ok(task) => {
                assert_eq!(expected_name, task.name);
                assert_eq!(Local::today(), task.time_limit);
            }
            Err(err) => {
                panic!(err);
            }
        }
    }

    #[test]
    fn test_create_task_invalid_timelimit() {
        let expected_name = "study".to_string();
        let expected_time_limit = create_date_str(Local::today() - Duration::days(1));
        match Task::new(&expected_name, &expected_time_limit.to_string()) {
            Ok(task) => panic!("{}", task.time_limit),
            Err(err) => assert_eq!("create task error.", err),
        }
    }

    fn create_date_str(date: Date<Local>) -> String {
        let date_str = date.to_string();
        let v: Vec<&str> = date_str.split("+").collect();
        let mut v_str = v[0].to_string();
        v_str.retain(|c| c != '-');
        return v_str;
    }
}
