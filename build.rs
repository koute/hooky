extern crate gcc;

fn main() {
    gcc::Config::new()
        .file( "native/core.c" )
        .file( "native/elfhacks.c" )
        .compile( "libcore.a" );
}
