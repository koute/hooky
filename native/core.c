#include <stdlib.h>
#include "elfhacks.h"

void * hooky_load_symbol( const char * library_name, const char * symbol ) {
    void * output = NULL;

    eh_obj_t library;
    if( eh_find_obj( &library, library_name ) ) {
	    return NULL;
    }

    if( eh_find_sym( &library, symbol, ( void ** )&output ) ) {
        return NULL;
    }

    return output;
}
