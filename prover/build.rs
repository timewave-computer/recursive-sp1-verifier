use sp1_build::build_program_with_args;

fn main() {
    build_program_with_args("../provable-program", Default::default());
    build_program_with_args("../recursive-program", Default::default());
    build_program_with_args("../recursive-arkworks-program", Default::default());
}
