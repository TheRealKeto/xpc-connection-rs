#include <xpc/xpc.h>

typedef struct _xpc_pipe_s *xpc_pipe_t;

struct _os_alloc_once_s
{
	long once;
	void *ptr;
};

extern struct _os_alloc_once_s _os_alloc_once_table[];

// XPC sets up global variables using os_alloc_once. By reverse engineering
// you can determine the values. The only one we actually need is the fourth
// one, which is used as an argument to xpc_pipe_routine

struct xpc_global_data
{
	uint64_t a;
	uint64_t xpc_flags;
	mach_port_t task_bootstrap_port; /* 0x10 */
#ifndef _64
	uint32_t padding;
#endif
	xpc_object_t xpc_bootstrap_pipe; /* 0x18 */
									 // and there's more, but you'll have to wait for MOXiI 2 for those...
									 // ...
};
