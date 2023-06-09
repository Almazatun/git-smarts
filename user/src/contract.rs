use gstd::{debug, prog::ProgramGenerator, ActorId, msg::{load, reply, source, send_for_reply}, exec::{random}, prelude::*};
use user_io::{UserActionRequest, UserActionResponse, UpdateUserDataInput, InitProgram, Repository, CreateRepositoryInput};
// use user_io::{Collaborator, RepoActionRequests, RepoActionResponses};
// use uuid::{Uuid};

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Program {
    pub owner: ActorId,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub repos: BTreeMap<ActorId, Repository>,
    pub repo_code_id: ActorId,
}

impl Program {
    fn new(init_program: InitProgram) -> Self {
        Self { 
            owner, 
            first_name: init_program.first_name, 
            last_name: init_program.last_name,
            username: init_program.username,
            repo_code_id: init_program.repo_code_id,
            repos: BTreeMap::new() 
        }
    }

    fn update_data(&mut self, update_input: UpdateUserDataInput) -> Self {
        self.first_name = update_input.first_name;
        self.last_name = update_input.last_name;
        self.username = update_input.username;

        self
    }

    async fn create_repo(&mut self, create_repo_input: CreateRepositoryInput) {
            let result = ProgramGenerator::create_program(
                self.repo_code_id,
                self.owner, 
                0,
            ).unwrap();

            self.repos.insert(result.1, Repository { id: result.1, name: create_repo_input.name })
        }

    fn is_exist_branch_by_name(&self, name: String) -> bool {
        for (_, repo) in self.repos.iter() {
            if repo.name == name {
                return true;
            }
        }
        
        false
    }
}

static mut CONTRACT: Option<Program> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let init_msg: InitProgram  = load().expect("Unable to decode init program");
    debug!("{:?} init program msg", init_msg);

    let program = Program::new(init_msg);

     unsafe { CONTRACT = Some(program)  }
}

#[no_mangle]
extern "C" fn handle() {
    let new_msg: ActionRequest = load().expect("Unable to decode `ActionRequest`");
    debug!("{:?} message", new_msg);

    let user_program = unsafe { CONTRACT.get_or_insert(Default::default()) };

    match new_msg {
        UserActionRequest::UpdateUserData(update_input) => {
            // user actor_id
            let actor_id = source();
            
            if actor_id != user_program.owner {
               panic!("Access denied") 
            }

            user_program.update_data(update_input);

            reply(UserActionResponse::UpdateUserData { message: "successfully update data".to_string()  }, 0)
            .expect("Unable to reply");
        }

        UserActionRequest::CreateRepository(create_repo_input) => {
            // user actor_id
            let actor_id = source();
            
            if actor_id != user_program.owner {
               panic!("Access denied") 
            }

            user_program.create_repo(create_repo_input);

            reply(UserActionResponse::CreateRepository { message: "Successfully create repository dapp".to_string() }, 0)
            .expect("Unable to reply");
        }

        UserActionRequest::RenameRepository(name) => {
            // user actor_id
            let actor_id = source();
            
            if actor_id != user_program.owner {
               panic!("Access denied") 
            }

            // TODO
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