/* automatically generated by rust-bindgen */

use bitflags::*;

pub type __s8 = ::libc::c_char;
pub type __u8 = ::libc::c_uchar;
pub type __s16 = ::libc::c_short;
pub type __u16 = ::libc::c_ushort;
pub type __s32 = ::libc::c_int;
pub type __u32 = ::libc::c_uint;
pub type __s64 = ::libc::c_longlong;
pub type __u64 = ::libc::c_ulonglong;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub fds_bits: [::libc::c_ulong; 16usize],
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

pub type __kernel_fd_set = Struct_Unnamed1;
pub type __kernel_sighandler_t = ::std::option::Option<extern "C" fn(arg1: ::libc::c_int) -> ()>;
pub type __kernel_key_t = ::libc::c_int;
pub type __kernel_mqd_t = ::libc::c_int;
pub type __kernel_old_uid_t = ::libc::c_ushort;
pub type __kernel_old_gid_t = ::libc::c_ushort;
pub type __kernel_old_dev_t = ::libc::c_ulong;
pub type __kernel_long_t = ::libc::c_long;
pub type __kernel_ulong_t = ::libc::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::libc::c_uint;
pub type __kernel_pid_t = ::libc::c_int;
pub type __kernel_ipc_pid_t = ::libc::c_int;
pub type __kernel_uid_t = ::libc::c_uint;
pub type __kernel_gid_t = ::libc::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::libc::c_int;
pub type __kernel_uid32_t = ::libc::c_uint;
pub type __kernel_gid32_t = ::libc::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub val: [::libc::c_int; 2usize],
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type __kernel_fsid_t = Struct_Unnamed2;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::libc::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::libc::c_int;
pub type __kernel_clockid_t = ::libc::c_int;
pub type __kernel_caddr_t = *mut ::libc::c_char;
pub type __kernel_uid16_t = ::libc::c_ushort;
pub type __kernel_gid16_t = ::libc::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type Enum_perf_type_id = ::libc::c_uint;
pub const PERF_TYPE_HARDWARE: ::libc::c_uint = 0;
pub const PERF_TYPE_SOFTWARE: ::libc::c_uint = 1;
pub const PERF_TYPE_TRACEPOINT: ::libc::c_uint = 2;
pub const PERF_TYPE_HW_CACHE: ::libc::c_uint = 3;
pub const PERF_TYPE_RAW: ::libc::c_uint = 4;
pub const PERF_TYPE_BREAKPOINT: ::libc::c_uint = 5;
pub const PERF_TYPE_MAX: ::libc::c_uint = 6;
pub type Enum_perf_hw_id = ::libc::c_uint;
pub const PERF_COUNT_HW_CPU_CYCLES: ::libc::c_uint = 0;
pub const PERF_COUNT_HW_INSTRUCTIONS: ::libc::c_uint = 1;
pub const PERF_COUNT_HW_CACHE_REFERENCES: ::libc::c_uint = 2;
pub const PERF_COUNT_HW_CACHE_MISSES: ::libc::c_uint = 3;
pub const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: ::libc::c_uint = 4;
pub const PERF_COUNT_HW_BRANCH_MISSES: ::libc::c_uint = 5;
pub const PERF_COUNT_HW_BUS_CYCLES: ::libc::c_uint = 6;
pub const PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: ::libc::c_uint = 7;
pub const PERF_COUNT_HW_STALLED_CYCLES_BACKEND: ::libc::c_uint = 8;
pub const PERF_COUNT_HW_REF_CPU_CYCLES: ::libc::c_uint = 9;
pub const PERF_COUNT_HW_MAX: ::libc::c_uint = 10;
pub type Enum_perf_hw_cache_id = ::libc::c_uint;
pub const PERF_COUNT_HW_CACHE_L1D: ::libc::c_uint = 0;
pub const PERF_COUNT_HW_CACHE_L1I: ::libc::c_uint = 1;
pub const PERF_COUNT_HW_CACHE_LL: ::libc::c_uint = 2;
pub const PERF_COUNT_HW_CACHE_DTLB: ::libc::c_uint = 3;
pub const PERF_COUNT_HW_CACHE_ITLB: ::libc::c_uint = 4;
pub const PERF_COUNT_HW_CACHE_BPU: ::libc::c_uint = 5;
pub const PERF_COUNT_HW_CACHE_NODE: ::libc::c_uint = 6;
pub const PERF_COUNT_HW_CACHE_MAX: ::libc::c_uint = 7;
pub type Enum_perf_hw_cache_op_id = ::libc::c_uint;
pub const PERF_COUNT_HW_CACHE_OP_READ: ::libc::c_uint = 0;
pub const PERF_COUNT_HW_CACHE_OP_WRITE: ::libc::c_uint = 1;
pub const PERF_COUNT_HW_CACHE_OP_PREFETCH: ::libc::c_uint = 2;
pub const PERF_COUNT_HW_CACHE_OP_MAX: ::libc::c_uint = 3;
pub type Enum_perf_hw_cache_op_result_id = ::libc::c_uint;
pub const PERF_COUNT_HW_CACHE_RESULT_ACCESS: ::libc::c_uint = 0;
pub const PERF_COUNT_HW_CACHE_RESULT_MISS: ::libc::c_uint = 1;
pub const PERF_COUNT_HW_CACHE_RESULT_MAX: ::libc::c_uint = 2;
pub type Enum_perf_sw_ids = ::libc::c_uint;
pub const PERF_COUNT_SW_CPU_CLOCK: ::libc::c_uint = 0;
pub const PERF_COUNT_SW_TASK_CLOCK: ::libc::c_uint = 1;
pub const PERF_COUNT_SW_PAGE_FAULTS: ::libc::c_uint = 2;
pub const PERF_COUNT_SW_CONTEXT_SWITCHES: ::libc::c_uint = 3;
pub const PERF_COUNT_SW_CPU_MIGRATIONS: ::libc::c_uint = 4;
pub const PERF_COUNT_SW_PAGE_FAULTS_MIN: ::libc::c_uint = 5;
pub const PERF_COUNT_SW_PAGE_FAULTS_MAJ: ::libc::c_uint = 6;
pub const PERF_COUNT_SW_ALIGNMENT_FAULTS: ::libc::c_uint = 7;
pub const PERF_COUNT_SW_EMULATION_FAULTS: ::libc::c_uint = 8;
pub const PERF_COUNT_SW_DUMMY: ::libc::c_uint = 9;
pub const PERF_COUNT_SW_MAX: ::libc::c_uint = 10;
pub type Enum_perf_branch_sample_type = ::libc::c_uint;
pub const PERF_SAMPLE_BRANCH_USER: ::libc::c_uint = 1;
pub const PERF_SAMPLE_BRANCH_KERNEL: ::libc::c_uint = 2;
pub const PERF_SAMPLE_BRANCH_HV: ::libc::c_uint = 4;
pub const PERF_SAMPLE_BRANCH_ANY: ::libc::c_uint = 8;
pub const PERF_SAMPLE_BRANCH_ANY_CALL: ::libc::c_uint = 16;
pub const PERF_SAMPLE_BRANCH_ANY_RETURN: ::libc::c_uint = 32;
pub const PERF_SAMPLE_BRANCH_IND_CALL: ::libc::c_uint = 64;
pub const PERF_SAMPLE_BRANCH_ABORT_TX: ::libc::c_uint = 128;
pub const PERF_SAMPLE_BRANCH_IN_TX: ::libc::c_uint = 256;
pub const PERF_SAMPLE_BRANCH_NO_TX: ::libc::c_uint = 512;
pub const PERF_SAMPLE_BRANCH_MAX: ::libc::c_uint = 1024;
pub type Enum_perf_sample_regs_abi = ::libc::c_uint;
pub const PERF_SAMPLE_REGS_ABI_NONE: ::libc::c_uint = 0;
pub const PERF_SAMPLE_REGS_ABI_32: ::libc::c_uint = 1;
pub const PERF_SAMPLE_REGS_ABI_64: ::libc::c_uint = 2;
pub type Enum_Unnamed3 = ::libc::c_ulong;
pub const PERF_TXN_ELISION: ::libc::c_ulong = 1;
pub const PERF_TXN_TRANSACTION: ::libc::c_ulong = 2;
pub const PERF_TXN_SYNC: ::libc::c_ulong = 4;
pub const PERF_TXN_ASYNC: ::libc::c_ulong = 8;
pub const PERF_TXN_RETRY: ::libc::c_ulong = 16;
pub const PERF_TXN_CONFLICT: ::libc::c_ulong = 32;
pub const PERF_TXN_CAPACITY_WRITE: ::libc::c_ulong = 64;
pub const PERF_TXN_CAPACITY_READ: ::libc::c_ulong = 128;
pub const PERF_TXN_MAX: ::libc::c_ulong = 256;
pub const PERF_TXN_ABORT_MASK: ::libc::c_ulong = 18446744069414584320;
pub const PERF_TXN_ABORT_SHIFT: ::libc::c_ulong = 32;

bitflags! {
    pub struct SampleFormatFlags: u64 {
        /// Records instruction pointer.
        const PERF_SAMPLE_IP = 1 << 0;
        /// Records the process and thread IDs.
        const PERF_SAMPLE_TID = 1 << 1;
        /// Records a timestamp.
        const PERF_SAMPLE_TIME = 1 << 2;
        /// Records an address, if applicable.
        const PERF_SAMPLE_ADDR = 1 << 3;
        /// Record counter values for all events in a group, not just the group leader.
        const PERF_SAMPLE_READ = 1 << 4;
        /// Records the callchain (stack backtrace).
        const PERF_SAMPLE_CALLCHAIN = 1 << 5;
        /// Records a unique ID for the opened event's group leader.
        const PERF_SAMPLE_ID = 1 << 6;
        /// Records CPU number.
        const PERF_SAMPLE_CPU = 1 << 7;
        /// Records the current sampling period.
        const PERF_SAMPLE_PERIOD = 1 << 8;
        /// Records  a  unique  ID  for  the  opened  event.  Unlike PERF_SAMPLE_ID the actual ID is returned, not the group
        /// leader.  This ID is the same as the one returned by PERF_FORMAT_ID.
        const PERF_SAMPLE_STREAM_ID = 1 << 9;
        /// Records additional data, if applicable.  Usually returned by tracepoint events.
        const PERF_SAMPLE_RAW = 1 << 10;
        /// This provides a record of recent branches, as provided by CPU branch  sampling  hardware  (such  as  Intel  Last
        /// Branch Record).  Not all hardware supports this feature.
        /// See the branch_sample_type field for how to filter which branches are reported.
        const PERF_SAMPLE_BRANCH_STACK = 1 << 11;
        /// Records the current user-level CPU register state (the values in the process before the kernel was called).
        const PERF_SAMPLE_REGS_USER = 1 << 12;
        /// Records the user level stack, allowing stack unwinding.
        const PERF_SAMPLE_STACK_USER = 1 << 13;
        /// Records a hardware provided weight value that expresses how costly the sampled event was.
        /// This allows the hardware to highlight expensive events in a profile.
        const PERF_SAMPLE_WEIGHT = 1 << 14;
        /// Records the data source: where in the memory hierarchy the data associated with the sampled instruction came from.
        /// This is only available if the underlying hardware supports this feature.
        const PERF_SAMPLE_DATA_SRC = 1 << 15;
        const PERF_SAMPLE_IDENTIFIER = 1 << 16;
        const PERF_SAMPLE_TRANSACTION = 1 << 17;
    }
}

bitflags! {
    pub struct ReadFormatFlags: u64 {
        /// Adds the 64-bit time_enabled field.  This can be used to calculate estimated totals if the PMU is overcommitted
        /// and multiplexing is happening.
        const FORMAT_TOTAL_TIME_ENABLED = 1 << 0;
        /// Adds the 64-bit time_running field.  This can be used to calculate estimated totals if the PMU is  overcommitted
        /// and  multiplexing is happening.
        const FORMAT_TOTAL_TIME_RUNNING = 1 << 1;
        /// Adds a 64-bit unique value that corresponds to the event group.
        const FORMAT_ID = 1 << 2;
        /// Allows all counter values in an event group to be read with one read.
        const FORMAT_GROUP = 1 << 3;
    }
}

bitflags! {
    pub struct EventAttrFlags: u64 {
                    /// off by default
                    const EVENT_ATTR_DISABLED       =  1 << 0;
                    /// children inherit it
                    const EVENT_ATTR_INHERIT        =  1 << 1;
                    /// must always be on PMU
                    const EVENT_ATTR_PINNED         =  1 << 2;
                    /// only group on PMU
                    const EVENT_ATTR_EXCLUSIVE      =  1 << 3;
                    /// don't count user
                    const EVENT_ATTR_EXCLUDE_USER   =  1 << 4;
                    /// ditto kernel
                    const EVENT_ATTR_EXCLUDE_KERNEL =  1 << 5;
                    /// ditto hypervisor
                    const EVENT_ATTR_EXCLUDE_HV     =  1 << 6;
                    /// don't count when idle
                    const EVENT_ATTR_EXCLUDE_IDLE   =  1 << 7;
                    /// include mmap data
                    const EVENT_ATTR_MMAP           =  1 << 8;
                    /// include comm data
                    const EVENT_ATTR_COMM           =  1 << 9;
                    /// use freq, not period
                    const EVENT_ATTR_FREQ           =  1 << 10;
                    /// per task counts
                    const EVENT_ATTR_INHERIT_STAT   =  1 << 11;
                    /// next exec enables
                    const EVENT_ATTR_ENABLE_ON_EXEC =  1 << 12;
                    /// trace fork/exit
                    const EVENT_ATTR_TASK           =  1 << 13;
                    /// wakeup_watermark
                    const EVENT_ATTR_WATERMARK      =  1 << 14;

                    /// SAMPLE_IP can have arbitrary skid
                    const EVENT_ATTR_SAMPLE_IP_ARBITRARY_SKID = 0 << 15;
                    /// SAMPLE_IP must have constant skid
                    const EVENT_ATTR_SAMPLE_IP_CONSTANT_SKID = 1 << 15;
                    /// SAMPLE_IP requested to have 0 skid
                    const EVENT_ATTR_SAMPLE_IP_REQ_ZERO_SKID = 2 << 15;
                    /// SAMPLE_IP must have 0 skid
                    const EVENT_ATTR_SAMPLE_IP_ZERO_SKID = 3 << 15;

                    /// non-exec mmap data
                    const EVENT_ATTR_MMAP_DATA =  1 << 17;
                    /// sample_type all events
                    const EVENT_ATTR_SAMPLE_ID_ALL =  1 << 18;
                    /// don't count in host
                    const EVENT_ATTR_EXCLUDE_HOST =  1 << 19;
                    /// don't count in guest
                    const EVENT_ATTR_EXCLUDE_GUEST =  1 << 20;
                    /// exclude kernel callchains
                    const EVENT_ATTR_EXCLUDE_CALLCHAIN_KERNEL = 1 << 21;
                    /// exclude user callchains
                    const EVENT_ATTR_EXCLUDE_CALLCHAIN_USER = 1 << 22;
                    /// include mmap with inode data
                    const EVENT_ATTR_MMAP2  =  1 << 23;
    }
}

pub type Enum_perf_event_ioc_flags = ::libc::c_uint;

pub const PERF_IOC_FLAG_GROUP: ::libc::c_uint = 1;

pub const PERF_EVENT_IOC_ENABLE: u64 = 9216;
pub const PERF_EVENT_IOC_DISABLE: u64 = 9217;
pub const PERF_EVENT_IOC_REFRESH: u64 = 9218;
pub const PERF_EVENT_IOC_RESET: u64 = 9219;
pub const PERF_EVENT_IOC_PERIOD: u64 = 1074275332;
pub const PERF_EVENT_IOC_SET_OUTPUT: u64 = 9221;
pub const PERF_EVENT_IOC_SET_FILTER: u64 = 1074275334;
pub const PERF_EVENT_IOC_ID: u64 = 2148017159;

pub type Enum_perf_event_type = ::libc::c_uint;
pub const PERF_RECORD_MMAP: ::libc::c_uint = 1;
pub const PERF_RECORD_LOST: ::libc::c_uint = 2;
pub const PERF_RECORD_COMM: ::libc::c_uint = 3;
pub const PERF_RECORD_EXIT: ::libc::c_uint = 4;
pub const PERF_RECORD_THROTTLE: ::libc::c_uint = 5;
pub const PERF_RECORD_UNTHROTTLE: ::libc::c_uint = 6;
pub const PERF_RECORD_FORK: ::libc::c_uint = 7;
pub const PERF_RECORD_READ: ::libc::c_uint = 8;
pub const PERF_RECORD_SAMPLE: ::libc::c_uint = 9;
pub const PERF_RECORD_MMAP2: ::libc::c_uint = 10;
pub const PERF_RECORD_MAX: ::libc::c_uint = 11;
pub type Enum_perf_callchain_context = ::libc::c_ulong;

pub const PERF_CONTEXT_HV: ::libc::c_ulong = 18446744073709551584;
pub const PERF_CONTEXT_KERNEL: ::libc::c_ulong = 18446744073709551488;
pub const PERF_CONTEXT_USER: ::libc::c_ulong = 18446744073709551104;
pub const PERF_CONTEXT_GUEST: ::libc::c_ulong = 18446744073709549568;
pub const PERF_CONTEXT_GUEST_KERNEL: ::libc::c_ulong = 18446744073709549440;
pub const PERF_CONTEXT_GUEST_USER: ::libc::c_ulong = 18446744073709549056;
pub const PERF_CONTEXT_MAX: ::libc::c_ulong = 18446744073709547521;
