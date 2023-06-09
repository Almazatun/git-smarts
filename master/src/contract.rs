use gstd::{debug, CodeId, prelude::*, ActorId, msg::{reply, source, load}, prog::ProgramGenerator};
use master_io::{InitProgram, ActionRequest, ActionResponse};

static mut CONTRACT: Option<Program> = None;

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Program {
    pub owner: ActorId,
    // <user_actor_id, user_actor_id>
    pub state:  BTreeMap<ActorId, ActorId>,
    // user program code id
    pub user_prog_code_id: CodeId,
}

impl Program {
    fn new(&self, init_program: InitProgram) -> Self {
        return Self { 
            user_prog_code_id: init_program.user_prog_code_id,
            state: BTreeMap::new(),
            owner: self.owner, 
        }
    }

    fn is_exist_user(&self, actor_id: ActorId) -> bool {
        return self.state.contains_key(&actor_id);
    }

    fn init_user_prog(&self) {
        ProgramGenerator::create_program(
            self.user_prog_code_id,
            self.owner, 
            0,
        ).unwrap();
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
                panic!("User already exists");
            }

            git_program.init_user_prog();
            git_program.state.insert(actor_id, actor_id);

            reply(ActionResponse::RegisterUser{ id: actor_id }, 0).expect("Unable to reply");
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