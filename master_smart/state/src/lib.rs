#![no_std]
use master_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{ActorId, prelude::*};

#[metawasm]
pub mod metafns {

    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn get_program_data(state: State) -> Program  {
        state
    }

    pub fn get_users(state: State) -> Vec<User> {
        let mut users: Vec<User> = vec![];

        for (_, user) in state.state {
            users.push(user)
        }

        users
    }

    pub fn get_user(state: State, actor_id: ActorId) -> Option<User> {
        if !state.state.contains_key(&actor_id) {
            panic!("User not found by actor id");
        }

        state.state.get(&actor_id).cloned()
    }
}