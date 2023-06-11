#![no_std]

use gmeta::{InOut, Metadata, In, TypeInfo};
use gstd::{ActorId,  prelude::*, CodeId};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitUserProgram>;
    type Handle = InOut<UserActionRequest, UserActionResponse>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Program;
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Program {
    pub owner: ActorId,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub repos: BTreeMap<ActorId, Repository>,
    pub repo_code_id: ActorId,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct InitUserProgram {
    pub owner: ActorId,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub repo_code_id: CodeId,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum UserActionRequest {
    UpdateUserData(UpdateUserDataInput),
    CreateRepository(CreateRepositoryInput),
    RenameRepository(String),
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum UserActionResponse {
    UpdateUserData{ message: String },
    CreateRepository{ message: String },
    Ok,
    Err
}


#[derive(Encode, Debug, Decode, TypeInfo)]
pub struct UpdateUserDataInput { 
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub struct CreateRepositoryInput { 
    pub name: String,
}

#[derive(Encode, Debug, Decode, TypeInfo, Clone)]
pub struct Repository { 
    pub id: ActorId,
    pub name: String,
}
