#![no_std]
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*};
use user_io::*;

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn get_program_data(state: State) -> Program  {
        state
    }

    pub fn get_user_repos(state: State) -> Vec<Repository> {
        let mut repos: Vec<Repository> = vec![];


        for (_, repo) in state.repos.iter() {
            repos.push(repo.clone())
        }

        repos
    }
}