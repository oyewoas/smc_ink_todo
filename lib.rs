#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod todo {
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    
    pub struct Task {
        id: u64,
        description: String,
        completed: bool,
    }

    #[ink(storage)]
    pub struct Todo {
        tasks: Mapping<u64, Task>,
        next_id: u64,
    }

    impl Todo {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tasks: Mapping::default(),
                next_id: 0,
            }
        }

        /// Default constructor
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// Add a new task
        #[ink(message)]
        pub fn add_task(&mut self, description: String) -> u64 {
            let id = self.next_id;
            let task = Task {
                id,
                description,
                completed: false,
            };
            self.tasks.insert(id, &task);
            let new_id = self.next_id.saturating_add(1);
            self.next_id = new_id;
            id
        }

        /// Get a task by id
        #[ink(message)]
        pub fn get_task(&self, id: u64) -> Option<Task> {
            self.tasks.get(id)
        }

        /// Get all tasks
        #[ink(message)]
        pub fn get_all_tasks(&self) -> Vec<Task> {
            let mut all = Vec::new();
            for i in 0..self.next_id {
                if let Some(task) = self.tasks.get(i) {
                    all.push(task);
                }
            }
            all
        }

        /// Mark a task as completed
        #[ink(message)]
        pub fn complete_task(&mut self, id: u64) -> bool {
            if let Some(mut task) = self.tasks.get(id) {
                task.completed = true;
                self.tasks.insert(id, &task);
                return true;
            }
            false
        }

        /// Update task description
        #[ink(message)]
        pub fn update_task(&mut self, id: u64, new_description: String) -> bool {
            if let Some(mut task) = self.tasks.get(id) {
                task.description = new_description;
                self.tasks.insert(id, &task);
                return true;
            }
            false
        }

        /// Delete a task
        #[ink(message)]
        pub fn delete_task(&mut self, id: u64) -> bool {
            if self.tasks.contains(id) {
                self.tasks.remove(id);
                return true;
            }
            false
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn add_and_get_task_works() {
            let mut todo = Todo::new();
            let id = todo.add_task("Learn ink!".into());
            let task = todo.get_task(id).unwrap();
            assert_eq!(task.description, "Learn ink!");
            assert_eq!(task.completed, false);
        }

        #[ink::test]
        fn complete_task_works() {
            let mut todo = Todo::new();
            let id = todo.add_task("Finish homework".into());
            assert_eq!(todo.complete_task(id), true);
            assert_eq!(todo.get_task(id).unwrap().completed, true);
        }

        #[ink::test]
        fn update_task_works() {
            let mut todo = Todo::new();
            let id = todo.add_task("Old task".into());
            assert_eq!(todo.update_task(id, "New task".into()), true);
            assert_eq!(todo.get_task(id).unwrap().description, "New task");
        }

        #[ink::test]
        fn delete_task_works() {
            let mut todo = Todo::new();
            let id = todo.add_task("Temporary task".into());
            assert_eq!(todo.delete_task(id), true);
            assert_eq!(todo.get_task(id), None);
        }

        #[ink::test]
        fn get_all_tasks_works() {
            let mut todo = Todo::new();
            todo.add_task("Task 1".into());
            todo.add_task("Task 2".into());
            let all = todo.get_all_tasks();
            assert_eq!(all.len(), 2);
        }
    }
}
