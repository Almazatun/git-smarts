#![no_std]
use gmeta::{InOut, Metadata, In, TypeInfo};
use gstd::{prog::ProgramGenerator, ActorId, CodeId, msg,  prelude::*};
// use chrono::{DateTime};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitProgram>;
    type Handle = InOut<ActionRequest, ActionResponse>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Program;
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Program {
    pub owner: ActorId,
    // <user_id, User>
    pub state:  BTreeMap<ActorId, User>,
    // profile code id
    pub repo_code_id: CodeId,
    pub branch_code_id: CodeId,
}

#[derive(Debug, TypeInfo, Decode, Encode)]
pub struct InitProgram {
    pub repo_code_id: CodeId,
    pub branch_code_id: CodeId,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum ActionRequest {
    RegisterUser(RegisterUserInput),
    UpdateUserData(UpdateUserDataInput),
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum ActionResponse {
    RegisterUser{ id: ActorId },
    UpdateUserData{ id: ActorId },
}

#[derive(Debug, TypeInfo, Decode, Encode)]
pub struct RegisterUserInput {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub repos_program_id: Option<ActorId>,
    pub branches_program_id: Option<ActorId>,
}

#[derive(Debug, TypeInfo, Decode, Encode)]
pub struct UpdateUserDataInput {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub owner: ActorId,
    pub repos_program_id: Option<ActorId>,
    pub branches_program_id: Option<ActorId>,
    // <repository_id, repository_id>
    pub repos: BTreeMap<String, String>,
    // <repository_id, branch_id>
    pub repo_branches: BTreeMap<String, String>,
    // pub created_at: DateTime,
    // pub updated_at: DateTime,
}

// const GAS_CREATE_REPOS_PROGRAM: u64 = 240_000_000_000;
// const GAS_CREATE_BRANCHES_PROGRAM: u64 = 240_000_000_000;

impl User {
    pub fn new(register_user_input: RegisterUserInput) -> User {
        let actor_id = msg::source();
        Self {
            first_name: register_user_input.first_name,
            last_name: register_user_input.last_name,
            username: register_user_input.username,
            repos_program_id: None,
            branches_program_id: None,
            owner: actor_id,
            repos: BTreeMap::new(),
            repo_branches: BTreeMap::new(),
        }
    }

    pub fn init_programs(&mut self, repo_code_id: CodeId, branches_code_id: CodeId) {
            let repos_res = ProgramGenerator::create_program(
                repo_code_id,
                self.owner, 
                0,
            ).unwrap();

            let branches_res = ProgramGenerator::create_program(
                branches_code_id,
                self.owner, 
                0,
            ).unwrap();

            self.repos_program_id = Some(repos_res.1);
            self.branches_program_id = Some(branches_res.1);
    }
}
