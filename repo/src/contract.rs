use gstd::{debug, ActorId, msg::{load, reply, source}, exec::{random}, prelude::*};
use repo_io::{
    Branch, 
    Commit, 
    InitProgram, 
    RepoActionRequests, 
    RepoActionResponses, 
    RenameBranchInput, 
    RenameBranchInput, 
    CreateBranchInput, 
    DeleteBranchInput,
};
// use uuid::{Uuid};

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Program {
    pub owner: ActorId,
    pub name: String,
    pub collaborator: BTreeMap<ActorId, ActorId>,
    pub branches:  BTreeMap<u32,  Branch>,
}

impl Program {
    fn new(owner: ActorId, name: String) -> Self {
        Self { 
            owner,
            name,
            collaborator: BTreeMap::new(),
            branches: BTreeMap::new(),
         }
    }

    fn is_exist_branch_by_name(&self, name: String) -> bool {
        for (_, br) in self.branches.inter() {
            if br.name == name {
                return true;
            }
        }
        
        false
    }

    fn is_exist_branch(&self, id: u32) -> bool {
        for (_, br) in self.branches.inter() {
            if br.id == id {
                return true;
            }
        }
        
        false
    }

    fn is_exist_collaborator(&self, actor_id: ActorId) -> bool {
        if let Some(c) = self.collaborator.get(&actor_id) {
            return true
        }

        false    
    }

    fn add_collaborator(&mut self, actor_id: ActorId) {
        self.collaborator.insert(actor_id, actor_id)
    }

    fn delete_collaborator(&mut self, actor_id: ActorId) {
        self.collaborator.remove(&actor_id)
    }

    fn add_branch(&mut self, create_branch_input: CreateBranchInput) {
        self.branches.insert(create_branch_input.id, Branch::new(create_branch_input));
    }

    fn rename_branch(&mut self, rename_branch_input: RenameBranchInput) {
        if let Some(branch) = self.branches.get_mut(&rename_branch_input.id) {
            if branch.id == rename_branch_input.id {
                branch.rename(rename_branch_input.name)

                // TODO send message to user contract
            }
        }
    }

    fn delete_branch(&mut self, delete_branch_input: DeleteBranchInput) {
        self.branches.remove(&delete_branch_input.branch_id)
    }

    fn is_exits_branch_by_name(&self, name: String) -> bool {
        for (_, branch) in self.branches.iter(){
            if branch.name == name {
                return true;
            }
        }

        false
    }

    fn push_commit(&mut self, branch_id: u32, commit: Commit) {
        if let Some(branch) = self.branches.get_mut(&rename_branch_input.id) {
            branch.add_commit(commit)
        }
    }
}

static mut CONTRACT: Option<Program> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let init_msg: InitProgram  = load().expect("Unable to decode init program");
    debug!("{:?} init program msg", init_msg);

    let program = Program::new(init_msg.owner, init_msg.name);

     unsafe { CONTRACT = Some(program)  }
}

#[no_mangle]
extern "C" fn handle() {
    let new_msg: RepoActionRequests = load().expect("Unable to decode `ActionRequest`");
    debug!("{:?} message", new_msg);

    let repo_program = unsafe { CONTRACT.get_or_insert(Default::default()) };

    match new_msg {
        RepoActionRequests::CreateBranch(name) => {
            let user_id = source();

            if user_id != repo_program.owner || !repo_program.is_exist_collaborator(user_id) {
                panic!("Access denied")
            }

            if repo_program.is_exist_branch_by_name(name) {
                panic!("Already exists branch by name")
            }

            let branch_input = CreateBranchInput { owner: user_id, id: gen_id(), name };
            repo_program.add_branch(branch_input);

           reply(RepoActionResponses::CreateBranch { msg: "Successfully create branch".to_string() }, 0)
           .expect("Unable to reply");
        }

        RepoActionRequests::RenameBranch(rename_branch_input) => {
            let user_id = source();
            let branch_id = rename_branch_input.id;

            if user_id != repo_program.owner || !repo_program.is_exist_collaborator(user_id) {
                panic!("Access denied")
            }

            if repo_program.is_exist_branch(branch_id) {
                panic!("Invalid branch id")
            }

            repo_program.rename_branch(rename_branch_input);

           reply(RepoActionResponses::RenameBranch { msg: "Successfully rename branch".to_string() }, 0)
           .expect("Unable to reply");
        }

        RepoActionRequests::DeleteBranch(delete_branch_input) => {
            let user_id = source();
        
            if user_id != repo_program.owner || !repo_program.is_exist_collaborator(user_id) {
                panic!("Access denied")
            }

            if repo_program.is_exist_branch(branch_id) {
                panic!("Invalid branch id")
            }

            repo_program.delete_branch(delete_branch_input);

           reply(RepoActionResponses::DeleteBranch { msg: "Successfully delete branch".to_string() }, 0)
           .expect("Unable to reply");
        }

        RepoActionRequests::Push(push_input) => {
            let user_id = source();
        
            if user_id != repo_program.owner || !repo_program.is_exist_collaborator(user_id) {
                panic!("Access denied")
            }

            if repo_program.is_exist_branch(push_input.branch_id) {
                panic!("Invalid branch id")
            }
            
            let commit = Commit{
                id: gen_id(), 
                owner: user_id, 
                message: push_input.message,
                hash: push_input.hash,
                description: push_input.description,
             };
            repo_program.push_commit(push_input.branch_id, commit);

           reply(RepoActionResponses::Push { msg: "Successfully commit".to_string() }, 0)
           .expect("Unable to reply");
        }

        RepoActionRequests::AddCollaborator(actor_id) => {
            let user_id = source();
        
            if user_id != repo_program.owner || !repo_program.is_exist_collaborator(user_id) {
                panic!("Access denied")
            }
            
            repo_program.add_collaborator(actor_id);

           reply(RepoActionResponses::AddCollaborator { msg: "Successfully add collaborator".to_string() }, 0)
           .expect("Unable to reply");
        }

        RepoActionRequests::DeleteCollaborator(actor_id) => {
            let user_id = source();
        
            if user_id != repo_program.owner || !repo_program.is_exist_collaborator(user_id) {
                panic!("Access denied")
            }
            
            repo_program.delete_collaborator(actor_id);

           reply(RepoActionResponses::AddCollaborator { msg: "Successfully delete collaborator".to_string() }, 0)
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