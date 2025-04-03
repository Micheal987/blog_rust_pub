mod addr;
mod router;
use router::main;
pub fn init_router() {
    let _ = main();
}
