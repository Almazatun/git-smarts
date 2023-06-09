#![no_std]
use gmeta::{metawasm, Metadata};
use gstd::{ActorId, prelude::*};
use repo_io::*;
#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn pull(state: State, branch_id: u32) -> Option<Branch> {
        if let Some(b) = state.branches.get(&branch_id) {
            return Some(b.clone())
        }

        None
    }

    pub fn get_collaborators(state: State) -> Vec<ActorId> {
        let mut response: Vec<ActorId> = vec![];

        for (_, c) in state.collaborator.iter() {
            response.push(c.clone())
        }

        response
    }
}