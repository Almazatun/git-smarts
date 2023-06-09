#![no_std]
use gmeta::{InOut, Metadata, In, TypeInfo};
use gstd::{ActorId, CodeId, prelude::*};
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
    // <user_actor_id, user_actor_id>
    pub state:  BTreeMap<ActorId, ActorId>,
    // user program code id
    pub user_prog_code_id: CodeId,
}

#[derive(Debug, TypeInfo, Decode, Encode)]
pub struct InitProgram {
    pub user_prog_code_id: CodeId,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum ActionRequest {
    RegisterUser(RegisterUserInput),
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum ActionResponse {
    RegisterUser{ id: ActorId },
}

#[derive(Debug, TypeInfo, Decode, Encode)]
pub struct RegisterUserInput {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

// #[derive(Encode, Decode, TypeInfo, Debug, Clone)]
// pub struct User {
//     pub username: String,
//     pub first_name: String,
//     pub last_name: String,
//     pub owner: ActorId,
//     pub program_id: ActorId,
//     // pub created_at: DateTime,
//     // pub updated_at: DateTime,
// }

// const GAS_CREATE_REPOS_PROGRAM: u64 = 240_000_000_000;
// const GAS_CREATE_BRANCHES_PROGRAM: u64 = 240_000_000_000;

// impl User {
//     pub fn new(register_user_input: RegisterUserInput) -> User {
//         let actor_id = msg::source();
//         Self {
//             first_name: register_user_input.first_name,
//             last_name: register_user_input.last_name,
//             username: register_user_input.username,
//             owner: actor_id,
//         }
//     }

//     pub fn init_programs(&mut self, repo_code_id: CodeId, branches_code_id: CodeId) {
//             let repos_res = ProgramGenerator::create_program(
//                 repo_code_id,
//                 self.owner, 
//                 0,
//             ).unwrap();

//             let repos_program_payload = InitBranchesProgram { owner: self.owner, repos_program_id: Some(repos_res.1) };

//             let branches_res = ProgramGenerator::create_program(
//                 branches_code_id,
//                 repos_program_payload.encode(),
//                 0,
//             ).unwrap();

//             self.repos_program_id = Some(repos_res.1);
//             self.branches_program_id = Some(branches_res.1);
//     }
// }
