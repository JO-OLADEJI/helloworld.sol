pub mod dispatcher;
pub mod handlers;
pub mod interface;

use dispatcher::dispatch_instruction;
use solana_program::entrypoint;

entrypoint!(dispatch_instruction);
