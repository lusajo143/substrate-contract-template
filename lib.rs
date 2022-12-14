#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod todo_contract {

    use core::iter::Map;

    use ink_env::caller;
    use ink_prelude::{
        string::{
            String,
            ToString as to_string,
        },
        vec::Vec
    };

    use scale::{Encode, Decode};

    use ink_storage::traits::{
        SpreadLayout, PackedLayout
    };

    mod users {
        use ink_prelude::{
            string:: {
                String
            }
        };
        use scale::{Encode, Decode};

        use ink_storage::traits::{
            SpreadLayout, PackedLayout
        };

        #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
        #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
        pub struct User {
            first_name: String,
            last_name: String,
            email: String,
            age: u32
        }

        impl User {
            pub fn new(first_name: String, last_name: String,
                email: String, age: u32) -> Self {
                    User {
                        first_name: first_name.clone(),
                        last_name: last_name.clone(),
                        email: email.clone(),
                        age: age
                    }
                }
            pub fn get_user(&self) -> User {
                let user: User = User {
                    first_name: self.first_name.clone(),
                    last_name: self.last_name.clone(),
                    email: self.email.clone(),
                    age: self.age
                };
                user
            }
            pub fn get_user_on_error() -> User {
                User {
                    first_name: String::from("null"),
                    last_name: String::from("null"),
                    email: String::from("null"),
                    age: 0
                }
            }
        }

    }

    /// Struct and its implementation for storing user tasks
    #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Task {
        task_name: String,
        task_date: String,
        is_done: bool
    }

    /// Traits
    trait TaskManagement {
        fn new() -> Self;
        fn on_error() -> Self;
    }

    impl TaskManagement for Task {
        fn new() -> Self {
            Task {
                task_name: String::from("Init"),
                task_date: String::from("Init"),
                is_done: false
            }
        }
        fn on_error() -> Task {
            Task {
                task_name: String::from("null"),
                task_date: String::from("null"),
                is_done: false
            }
        }
    }


    /// Responses
    #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct BaseResponse {
        code: u32,
        message: String
    }
    #[derive(Debug, Clone, Encode, Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct DataResponse<T> {
        code: u32,
        message: String,
        data: T
    }
    enum StatusCodes {
        CREATED(String),
        UPDATED(String),
        NULL(String),
        INTERNAL_ERROR(String)
    }
    pub const OPERATION_STATUS: &str = "Operation successfully!";
    pub const CREATED_STATUS: &str = "Created successfully!";
    pub const UPDATED_STATUS: &str = "Updated successfully!";
    pub const NULL_STATUS: &str = "Null Argument!";
    pub const NOT_FOUND_STATUS: &str = "Not found";
    pub const DUPLICATE_STATUS: &str = "Duplicate data!";
    pub const INTERNAL_ERROR_STATUS: &str = "Internal Constract Error!";
    
    pub const SUCCESS_CODE: u32 = 9000;
    pub const NULL_CODE: u32 = 9001;
    pub const NOT_FOUND_CODE: u32 = 9002;
    pub const DUPLICATE_CODE: u32 = 9003;
    pub const INTERNAL_ERROR_CODE: u32 = 9005;




    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct TodoContract {
        user: Mapping<AccountId, users::User>,
        tasks: Mapping<AccountId, Vec<Task>>
    }

    impl TodoContract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            first_name: String, last_name: String, email: String, age: u32
        ) -> Self {
            ink_lang::utils::initialize_contract(|todoContract: &mut Self| {
                let admin: users::User = users::User::new(
                    first_name,
                    last_name,
                    email,
                    age
                );
                todoContract.user.insert(Self::env().caller(), &admin)
            })
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            String::from("Works")
        }

        #[ink(message)]
        pub fn get_my_account(&self) -> DataResponse<users::User> {
            match self.user.get(Self::env().caller()) {
                Some(data) => {
                    DataResponse { code: SUCCESS_CODE, message: String::from(OPERATION_STATUS), data }
                    
                },
                None => {
                    DataResponse { code: NOT_FOUND_CODE, message: String::from(NOT_FOUND_STATUS), data: users::User::get_user_on_error() } 
                }
            }
            
        }

        #[ink(message)]
        pub fn register_user(&mut self,
            first_name: String,
            last_name: String,
            email: String,
            age: u32) -> BaseResponse {
                match self.user.get(Self::env().caller()) {
                    Some(user) => {
                        BaseResponse {
                            code: DUPLICATE_CODE,
                            message: String::from(DUPLICATE_STATUS)
                        }
                    },
                    None => {
                        let user = users::User::new(
                            first_name,
                            last_name,
                            email,
                            age
                        );
                        let mut task = Task::new();
                        task.task_name = String::from("Init");
                        task.task_date = String::from("Init");
                        task.is_done = false;

                        let mut initial_task: Vec<Task> = Vec::new();
                        initial_task.push(task);

                        self.user.insert(Self::env().caller(), &user);
                        self.tasks.insert(Self::env().caller(), &initial_task);
                        BaseResponse {
                            code: SUCCESS_CODE,
                            message: String::from(CREATED_STATUS)
                        }
                    }
                }
            }

        #[ink(message)]
        pub fn add_task(&mut self, task_name: String,
            task_date: String) -> BaseResponse {
            match self.tasks.get(Self::env().caller()) {
                Some(mut my_tasks) => {
                    let mut task = Task::new();
                    task.task_name = task_name.clone();
                    task.task_date = task_date.clone();
                    task.is_done = false;
                    my_tasks.push(task);
                    self.tasks.remove(Self::env().caller());
                    self.tasks.insert(Self::env().caller(), &my_tasks);
                    BaseResponse {
                        code: SUCCESS_CODE,
                        message: String::from(CREATED_STATUS)
                    }
                },
                None => {
                    BaseResponse {
                        code: INTERNAL_ERROR_CODE,
                        message: String::from(INTERNAL_ERROR_STATUS)
                    }
                }
            }
        }

        #[ink(message)]
        pub fn get_my_tasks(&self) -> DataResponse<Vec<Task>> {
            match self.tasks.get(Self::env().caller()) {
                Some(tasks) => {
                    DataResponse {
                        code: SUCCESS_CODE,
                        message: String::from(OPERATION_STATUS),
                        data: tasks
                    }
                },
                None => {
                    let tasks: Vec<Task> = Vec::new();
                    DataResponse {
                        code: INTERNAL_ERROR_CODE,
                        message: String::from(NOT_FOUND_STATUS),
                        data: tasks
                    }
                }
            }
        }

    }

    
}
