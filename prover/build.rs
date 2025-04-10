use sp1_build::build_program_with_args;

fn main() {
    build_program_with_args("../provable-program", Default::default());
    build_program_with_args("../recursive-program", Default::default());
    build_program_with_args("../recursive-arkworks-program", Default::default());
    build_program_with_args("../simple-merkle-proofs", Default::default());
    build_program_with_args("../smt-opening-proofs", Default::default());
}
