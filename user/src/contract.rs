use gstd::{
    debug, 
    prog::ProgramGenerator, 
    ActorId, 
    msg::{load, source, reply}, 
    prelude::*, 
    CodeId,
    exec::block_timestamp,
};
use user_io::{UserActionRequest, UserActionResponse, UpdateUserDataInput, InitUserProgram, Repository};
use repo_io::InitRepoProgram;
// use uuid::{Uuid};

#[derive(Default, Encode, Decode, TypeInfo, Debug, Clone)]
pub struct Program {
    pub owner: ActorId,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub repos: BTreeMap<ActorId, Repository>,
    pub repo_code_id: CodeId,
}

impl Program {
    fn new(init_program: InitUserProgram) -> Self {
        Self { 
            owner: init_program.owner,
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

        self.clone()
    }

    fn create_repo(&mut self, create_repo: InitRepoProgram) {
            let result = ProgramGenerator::create_program(
                self.repo_code_id,
                create_repo.encode(), 
                0,
            ).unwrap();

            self.repos.insert(result.1, Repository { 
                id: result.1, 
                name: create_repo.name,
                created_at: block_timestamp(),
                updated_at: block_timestamp(),
             });
        }

    fn rename_repo(&mut self, repo_id: ActorId, name: String) {
        if let Some(repo) = self.repos.get_mut(&repo_id) {
            repo.name = name
        }
    }

    fn get_repo(&self, repo_id: ActorId) -> Option<Repository> {
        if let Some(repo) = self.repos.get(&repo_id).cloned() {
            return Some(repo)
        } else {
            return None
        }
    }

    fn get_repo_by_name(&self, name: String) -> Option<Repository> {
        for (_, r) in self.repos.iter() {
            if r.name == name {
                return Some(r.clone())
            }
        }

        None
    }
}

static mut CONTRACT: Option<Program> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let init_msg: InitUserProgram  = load().expect("Unable to decode init program");
    debug!("{:?} init program msg", init_msg);

    let program = Program::new(init_msg);

     unsafe { CONTRACT = Some(program)  }
}

#[no_mangle]
extern "C" fn handle() {
    let new_msg: UserActionRequest = load().expect("Unable to decode `ActionRequest`");
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
            
            user_program.create_repo(InitRepoProgram { 
                owner: actor_id, 
                name: create_repo_input.name
            });

            reply(UserActionResponse::CreateRepository { message: "Successfully create repository dapp".to_string() }, 0)
            .expect("Unable to reply");
        }

        UserActionRequest::RenameRepository(name) => {
            let actor_id = source();
            let repo_by_name = user_program.get_repo_by_name(name.clone());

            match repo_by_name {
                Some(repo_by_name) => {
                    if repo_by_name.id != actor_id {
                        panic!("Already exists repository by name")
                    }
                }
                None => ()
            }

            let repo = user_program.get_repo(actor_id);

            match repo {
                Some(repo) => {
                    if repo.id == actor_id {
                        user_program.rename_repo(actor_id, name);
                    } else {
                        reply(UserActionResponse::Err, 0).expect("Unable to reply");
                    }
                }
                None => ()
            }

            reply(UserActionResponse::Ok, 0)
            .expect("Unable to reply");
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