use gstd::{debug, ActorId, msg::{load, reply, source}, exec::{random}, prelude::*};
use repos_io::{Repository, ActionRequest, ActionResponse, Collaborator};
// use uuid::{Uuid};

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Program {
    pub owner: ActorId,
    // <repository_id, Repository>
    pub state:  BTreeMap<u32, Repository>
}

impl Program {
    fn new(owner: ActorId) -> Self {
        Self { owner: owner, state: BTreeMap::new() }
    }

    fn is_exist_repo_by_name(&self, name: String) -> bool {
        for (key, repo) in self.state.clone().into_iter() {
            if repo.name == name {
                return true;
            }
        }
        
        false
    }

    fn is_exist_repo(&self, id: u32) -> bool {
        if let Some(repo) = self.state.get(&id) {
            return true;
        } else {
            return  false;
        }
    }
}

static mut CONTRACT: Option<Program> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let init_msg: ActorId  = load().expect("Unable to decode init program");
    debug!("{:?} init program msg", init_msg);

    let program = Some(Program::new(init_msg));

     unsafe { CONTRACT = program  }
}

#[no_mangle]
extern "C" fn handle() {
    let new_msg: ActionRequest = load().expect("Unable to decode `ActionRequest`");
    debug!("{:?} message", new_msg);

    let repos_program = unsafe { CONTRACT.get_or_insert(Default::default()) };

    match new_msg {
        ActionRequest::CreateRepository(name) => {
            // user actor_id
            let actor_id = source();

            if repos_program.owner != actor_id {
                panic!("Access denied")
            }

            if repos_program.is_exist_repo_by_name(name.clone()) {
               panic!("Already exists repository by `name`") 
            }

            let repo_id = gen_id();
            let repository = Repository::new(name, repo_id);

            repos_program.state.insert(repo_id, repository);

            reply(ActionResponse::CreateRepository { repo_id }, 0).expect("Unable to reply");
        }

        ActionRequest::UpdateRepository(update_repo_input) => {
            let actor_id = source();

            if repos_program.owner != actor_id {
                panic!("Access denied")
            }

            if repos_program.owner != actor_id {
                panic!("Access denied")
            }

            if !repos_program.is_exist_repo(update_repo_input.id) {
                panic!("Invalid repository id")
            }

            repos_program.state.entry(update_repo_input.id).and_modify(|repo| {
                repo.name = update_repo_input.name;
            });

            reply(ActionResponse::UpdateRepository { repo_id: update_repo_input.id }, 0).expect("Unable to reply");
        }

        ActionRequest::AddCollaborator(add_collaborator_input) => {
            let user_id = add_collaborator_input.id;
            let actor_id = source();

            if repos_program.owner != actor_id {
                panic!("Access denied")
            }

            if !repos_program.is_exist_repo(add_collaborator_input.repo_id) {
                panic!("Invalid repository id")
            }
            
            repos_program.state
            .entry(add_collaborator_input.repo_id)
            .and_modify(|repo| {
                if repo.is_exist_collaborator(user_id) {
                    panic!("User already exists in repository")
                }


                let collaborator = Collaborator { id: user_id };
                repo.collaborators.push(collaborator);
            });

            reply(ActionResponse::AddCollaborator { actor_id: add_collaborator_input.id }, 0).expect("Unable to reply");
        }

        ActionRequest::DeleteCollaborator(delete_collaborator_input) => {
            let collaborator_id = delete_collaborator_input.id;
            let repo_id = delete_collaborator_input.repo_id;
            let actor_id = source();

            if repos_program.owner != actor_id {
                panic!("Access denied")
            }

            if !repos_program.is_exist_repo(repo_id) {
                panic!("Invalid repository id")
            }

            if let Some(repo) = repos_program.state.get(&repo_id) {
              if !repo.is_exist_collaborator(collaborator_id) {
                panic!("Invalid collaborator id")
              } else {
                // TODO how fix
                repo.delete_collaborator(collaborator_id);
              }
            } 

            reply(ActionResponse::DeleteCollaborator { actor_id: collaborator_id }, 0).expect("Unable to reply");
        }

        ActionRequest::DeleteRepository(repo_id) => {
            let actor_id = source();

            if repos_program.owner != actor_id {
                panic!("Access denied")
            }

            if !repos_program.is_exist_repo(repo_id) {
                panic!("Invalid repository id")
            }

            repos_program.state.remove(&repo_id);

            reply(ActionResponse::DeleteRepository { repo_id }, 0).expect("Unable to reply");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let program = unsafe { CONTRACT.get_or_insert(Default::default()) };
    reply(program, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    reply(metahash, 0).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

static mut SEED: u8 = 0;

fn gen_id() -> u32 {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED + 1 };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = random(random_input).expect("Error in getting random number");
    let bytes = [random[0], random[1], random[2], random[3]];
    u32::from_be_bytes(bytes)
}

// fn gen_id() -> String {
//     Uuid::new_v4().to_string()
// }