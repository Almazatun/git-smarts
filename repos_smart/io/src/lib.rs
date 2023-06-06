#![no_std]
use gmeta::{InOut, Metadata, In, TypeInfo};
use gstd::{ActorId,  prelude::*};
use rand::{distributions::Alphanumeric, Rng};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<ActorId>; //owner
    type Handle = InOut<ActionRequest, ActionResponse>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Program;
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Program {
    pub owner: ActorId,
    // <repository_id, Repository>
    pub state:  BTreeMap<String, Repository>
}

#[derive(Debug, TypeInfo, Decode, Encode)]
pub struct InitProgram {
    pub owner: ActorId,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum ActionRequest {
    CreateRepository(String),
    UpdateRepository(UpdateRepositoryInput),
    DeleteRepository(String),
    AddCollaborator(AddCollaboratorInput),
    DeleteCollaborator(DeleteCollaboratorInput),
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub enum ActionResponse {
    CreateRepository{ repo_id: String },
    UpdateRepository{ repo_id: String },
    DeleteRepository{ repo_id: String },
    AddCollaborator{ actor_id: ActorId },
    DeleteCollaborator{ actor_id: ActorId },
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub struct UpdateRepositoryInput {
    pub id: String,
    pub name: String,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub struct AddCollaboratorInput {
    pub id: ActorId,
    pub repo_id: String,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
pub struct DeleteCollaboratorInput {
    pub id: ActorId,
    pub repo_id: String,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub struct Repository {
   pub id: String,
   pub name: String,
   pub collaborators: Vec<Collaborator>,
   // pub created_at: DateTime,
   // pub updated_at: DateTime,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
pub struct Collaborator {
   pub id: ActorId,
   // pub created_at: DateTime,
}

impl Repository {
    pub fn new(name: String) -> Self {
        let id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

        Self { id, name, collaborators: vec![] }
    }

    pub fn rename(mut self, new_name: String) -> Self {
        self.name = new_name;

        self
    }

    pub fn is_exist_collaborator(&self, id: ActorId) -> bool {
        if let Some(_) = self.collaborators.iter().position(|collaborator| collaborator.id == id) {
            return true;
        } else {
            return false;
        }
    }

    pub fn add_collaborator(&mut self, actor_id: ActorId) -> Collaborator {
        let collaborator = Collaborator { id: actor_id };
        self.collaborators.push(collaborator);

        self.collaborators.last().unwrap().clone()
    }

    pub fn delete_collaborator(&mut self, actor_id: ActorId) -> ActorId {
        if !self.is_exist_collaborator(actor_id) {
            panic!("Invalid collaborator id")
        }

        self.collaborators.retain(|collaborator| collaborator.id != actor_id);

        actor_id
    }

    pub fn clear_collaborator(&mut self, actor_id: ActorId) -> ActorId {
        if !self.is_exist_collaborator(actor_id) {
            panic!("Invalid collaborator id")
        }

        self.collaborators.retain(|collaborator| collaborator.id != actor_id);

        actor_id
    }

    pub fn clear_collaborators(&mut self) {
        self.collaborators = vec![];
    }

}
