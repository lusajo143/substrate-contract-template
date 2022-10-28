#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;



#[ink::contract]
mod todo_contract {

    use ink_prelude::{
        string::{
            String,
            ToString as to_string,
        },
        vec::Vec
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
        }

    }

    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct TodoContract {
        user: Mapping<AccountId, users::User>
        
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

        // #[ink(message)]
        // pub fn get_my_account(&self) -> users::User {
        //     let 
        // }

    }

    
}
