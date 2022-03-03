use glicol::Engine;
// use glicol::GlicolNodeInfo;
// use std::collections::HashMap;

fn main() {
    let mut engine = Engine::<128>::new();
    engine.update("o: constsig 42 >>mul ~am; ~am: constsig 0.3");
    for e in engine.context.graph.edges(engine.context.destination) {
        println!("destinations {:?}", e);
    }
    engine.next_block();
    // engine.send_msg("o", 0, (0, "1."));
    // engine.next_block();
}