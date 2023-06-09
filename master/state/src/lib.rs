#![no_std]
use gmeta::{metawasm, Metadata};
use gstd::{ActorId, prelude::*};
use master_io::{Program, ProgramMetadata};

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn get_program_data(state: State) -> Program  {
        state
    }

    pub fn get_users(state: State) -> Vec<ActorId> {
        let mut user_actor_ids: Vec<ActorId> = vec![];

        for (_, id) in state.state {
            user_actor_ids.push(id)
        }

        user_actor_ids
    }

    pub fn get_user(state: State, actor_id: ActorId) -> Option<ActorId> {
        if !state.state.contains_key(&actor_id) {
            panic!("User not found by actor id");
        }

        state.state.get(&actor_id).cloned()
    }
}