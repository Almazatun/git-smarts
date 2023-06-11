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
    pub repo_prog_code_id: CodeId,
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
    pub owner: Option<ActorId>,
}