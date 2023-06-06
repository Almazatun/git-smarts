use gstd::{debug, CodeId, prelude::*, ActorId, msg::{reply, source, load}};
use master_io::{InitProgram, User, ActionRequest, ActionResponse, UpdateUserDataInput};

static mut CONTRACT: Option<Program> = None;

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Program {
    pub owner: ActorId,
    // <user_id, User>
    pub state:  BTreeMap<ActorId, User>,
    // profile code id
    pub repo_code_id: CodeId,
    pub branch_code_id: CodeId,
}

impl Program {
    fn new(&self, init_program: InitProgram) -> Self {
        return Self { 
            repo_code_id: init_program.repo_code_id,
            branch_code_id: init_program.branch_code_id,
            state: BTreeMap::new(),
            owner: self.owner, 
        }
    }

    fn is_exist_user(&self, actor_id: ActorId) -> bool {
        return self.state.contains_key(&actor_id);
    }

    fn update_user_data(
        &mut self, 
        actor_id: ActorId, 
        update_user_data_input: UpdateUserDataInput,
    ) -> ActorId {
        if let Some(user) = self.state.get_mut(&actor_id) {
            user.first_name = update_user_data_input.first_name;
            user.last_name = update_user_data_input.last_name;
            user.username = update_user_data_input.username;
        }

        return actor_id
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_program_data: InitProgram  = load().expect("Unable to decode init program");
    debug!("{:?} init program", init_program_data);
    let init_program = Program::new(
        &Default::default(),
        init_program_data,
    );

     CONTRACT = Some(init_program);
}

#[no_mangle]
extern "C" fn handle() {
    let new_msg: ActionRequest = load().expect("Unable to decode `ActionRequest`");
    debug!("{:?} message", new_msg);

    let git_program = unsafe { CONTRACT.get_or_insert(Default::default()) };

    match new_msg {
        ActionRequest::RegisterUser(register_user_input) => {
            // user actor_id
            let actor_id = source();
            
            if git_program.is_exist_user(actor_id) {
                panic!("User already exists by actor id");
            }
            
            let mut user = User::new(register_user_input);
            let repo_code_id = git_program.repo_code_id;
            let branches_code_id = git_program.branch_code_id;

            user.init_programs(repo_code_id, branches_code_id);

            git_program.state.insert(actor_id, user);

            reply(ActionResponse::RegisterUser{ id: actor_id }, 0).expect("Unable to reply");
        }

        ActionRequest::UpdateUserData(update_user_data_input) => {
            // user actor_id
            let actor_id = source();

            if !git_program.is_exist_user(actor_id) {
                panic!("User not found by actor id");
            }
            
            git_program.update_user_data(actor_id, update_user_data_input);

            reply(ActionResponse::UpdateUserData { id: actor_id }, 0).expect("Unable to reply");
        }
    };
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