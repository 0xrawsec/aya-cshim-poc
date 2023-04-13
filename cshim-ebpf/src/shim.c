#include <linux/types.h>
#include <sys/types.h>

// uncomment if BPF_CORE_READ must be used
//#include <bpf/bpf_helpers.h>
//#include <bpf/bpf_core_read.h>

/*
IMPORTANT: it seems defining and using typedefs (for structs) in shim
makes it fail at linking, so don't do it.
*/

// this just a simple C macro to make easier shim definition
#define SHIM(proto, accessed_member)                                     \
	__attribute__((always_inline)) proto                             \
	{                                                                \
		return __builtin_preserve_access_index(accessed_member); \
	}

struct kgid_t {
	gid_t val;
} __attribute__((preserve_access_index));

struct kuid_t {
	uid_t val;
} __attribute__((preserve_access_index));

// Defining shim for cred struct
// We just need to define the fields we need to access

struct cred {
	struct kuid_t uid;
	struct kgid_t gid;
} __attribute__((preserve_access_index));

SHIM(uid_t cred_uid(struct cred *pcred), pcred->uid.val);
SHIM(gid_t cred_gid(struct cred *pcred), pcred->uid.val);

// Defining shim for task_struct
// We just need to define the fields we need to access

struct task_struct {
	pid_t pid;
	__u64 start_time;
	__u64 start_boottime;
	pid_t tgid;
	char comm[16];
	struct cred *cred; // gives an example of nested access
} __attribute__((preserve_access_index));

SHIM(__u64 task_struct_start_time(struct task_struct *task), task->start_time);
SHIM(__u64 task_struct_start_boottime(struct task_struct *task),
     task->start_boottime);
SHIM(char *task_struct_comm(struct task_struct *task), &task->comm[0]);
SHIM(pid_t task_struct_pid(struct task_struct *task), task->pid);
SHIM(pid_t task_struct_tgid(struct task_struct *task), task->tgid);
SHIM(struct cred *task_struct_cred(struct task_struct *task), task->cred);
SHIM(uid_t task_struct_cred_uid(struct task_struct *task), task->cred->uid.val);