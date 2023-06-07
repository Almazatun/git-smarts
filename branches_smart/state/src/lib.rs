#![no_std]
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*};
use repos_io::*;
#[metawasm]
pub mod metafns {

    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn get_program_data(state: State) -> Program  {
        state
    }

    pub fn get_repositories(state: State) -> Vec<Repository>  {
        let mut repos = vec![];

        for (_, repo) in state.state {
            repos.push(repo)
        }

        repos
    }

    pub fn get_repository(state: State, id: u32) -> Option<Repository>  {
        state.state.get(&id).cloned()
    }
}