fn main() {
    cc::Build::new()
        .file( "native/core.c" )
        .file( "native/elfhacks.c" )
        .compile( "libcore.a" );
}
