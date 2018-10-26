pub mod sys;

use std::os::unix::io::{
    IntoRawFd,
    RawFd
};
pub mod socket {
    use nix::libc;
    use std::result::Result;
    use super::sys;
    use std::ptr::NonNull;
    use std::marker::PhantomData;
    use std::os::unix::prelude::*;
    use num_traits::ToPrimitive;

    #[derive(FromPrimitive, ToPrimitive, Debug)]
    #[repr(i32)]
    pub enum Family {
        /// Receives routing and link updates and may be used to modify
        /// the routing tables (both IPv4 and IPv6), IP addresses, link
        /// parameters, neighbor setups, queueing disciplines, traffic
        /// classes and packet classifiers (see rtnetlink(7)).
        Route = libc::NETLINK_ROUTE,
        /// Reserved for user-mode socket protocols.
        UserSock = libc::NETLINK_USERSOCK,
        /// Transport IPv4 packets from netfilter to user space.  Used by
        /// ip_queue kernel module.  After a long period of being declared
        /// obsolete (in favor of the more advanced nfnetlink_queue
        /// feature), NETLINK_FIREWALL was removed in Linux 3.5.
        Firewall = libc::NETLINK_FIREWALL,
        /// Query information about sockets of various protocol families
        /// from the kernel (see sock_diag(7))
        InetDiag = libc::NETLINK_INET_DIAG,
        /// Netfilter/iptables ULOG.
        Nflog = libc::NETLINK_NFLOG,
        /// IpSec.
        Xfrm = libc::NETLINK_XFRM,
        /// SELinux event notifications.
        SeLinux = libc::NETLINK_SELINUX,
        /// Open-iCSI.
        Iscsi = libc::NETLINK_ISCSI,
        /// Auditing.
        Audit = libc::NETLINK_AUDIT,
        /// Access to FIB lookup from user space.
        FibLookup = libc::NETLINK_FIB_LOOKUP,
        /// Kernel connector  See Documentation/connector/* in the Linux
        /// kernel source tree for further information.
        Connector = libc::NETLINK_CONNECTOR,
        /// Netfilter subsystem.
        NetFilter = libc::NETLINK_NETFILTER,
        ///  Transport IPv6 packets from netfilter to user space.
        IP6FW = libc::NETLINK_IP6_FW,
        /// DECNet routing messages
        DECnetRoutingMessages = libc::NETLINK_DNRTMSG,
        /// Kernel messages to user-space
        KernelMessages = libc::NETLINK_KOBJECT_UEVENT,
        /// Generic netlink family for simplified netlink usage
        Generic = libc::NETLINK_GENERIC,
        /// Netlink interace to request information about ciphers
        /// registered with the kernel crypto API as well as allow
        /// configuration of the kernel crypto API
        Crypto = libc::NETLINK_CRYPTO,
    }
    pub type RawSocket = sys::nl_sock;
    pub struct Socket {
        ptr: NonNull<RawSocket>,
        phantom: PhantomData<RawSocket>
    }
    impl Socket {
        pub fn new() -> Self {
            Self::try_new().unwrap()
        }

        pub fn try_new() -> Option<Self> {
            unsafe {
                Self::from_raw(
                    sys::nl_socket_alloc()
                )
            }
        }

        pub unsafe fn from_raw(ptr: *mut RawSocket) -> Option<Self> {
            NonNull::new(ptr).map(|ptr| {
                Socket {
                    ptr,
                    phantom: PhantomData
                }
            })
        }

        pub fn connect(&mut self, family: Family) -> Result<(), i32> {
            unsafe {
                let err = sys::nl_connect(self.ptr.as_ptr(), family.to_i32().unwrap());
                if err < 0 {
                    Err(err)
                } else {
                    Ok(())
                }
            }
        }
        pub fn as_ptr(&mut self) -> *mut RawSocket {
            self.ptr.as_ptr()
        }
    }
    impl AsRawFd for Socket {
        fn as_raw_fd(&self) -> RawFd {
            unsafe {
                sys::nl_socket_get_fd(self.ptr.as_ptr())
            }
        }
    }
}
pub mod message {
    use std::marker::PhantomData;
    use std::ptr::NonNull;
    use super::sys;
    use nix::libc;
    use self::libc::{
        c_int
    };
    pub type RawMessageHeader = sys::nlmsghdr;
    pub type RawMessage = sys::nl_msg;
    pub struct MessageType(::std::os::raw::c_int);
    impl MessageType {
        pub unsafe fn from_c_int(message_type: ::std::os::raw::c_int) -> MessageType {
            MessageType(message_type)
        }
        pub fn to_c_int(&self) -> c_int {
            self.0
        }
    }

    pub struct Message {
        ptr: NonNull<RawMessage>,
        phantom: PhantomData<RawMessage>
    }


    impl Message {
        pub fn new() -> Self { Self::try_new().unwrap() }
        pub fn with_size(size: usize) -> Self { Self::try_with_size(size).unwrap() }
        pub fn with_type(message_type: MessageType, flags: c_int) -> Self { Self::try_with_type(message_type, flags).unwrap()}

        pub fn try_new() -> Option<Self> {
            Self::from_raw(unsafe {
                sys::nlmsg_alloc()
            })
        }


        pub fn try_with_size(size: usize) -> Option<Self> {
            Self::from_raw(unsafe {
                sys::nlmsg_alloc_size(size)
            })
        }

        pub fn try_with_type(message_type: MessageType, flags: c_int) -> Option<Self> {
            Self::from_raw(unsafe {
                sys::nlmsg_alloc_simple(
                    message_type.to_c_int(),
                    flags
                )
            })
        }

        pub fn from_raw(ptr : *mut RawMessage) -> Option<Message> {
            NonNull::new(ptr).map(|ptr| {
                Message { ptr, phantom: PhantomData }
            })
        }

        pub fn set_default_size(size : usize) {
            unsafe {
                super::sys::nlmsg_set_default_size(size)
            }
        }
        pub fn as_ptr(&mut self) -> *mut RawMessage {
            self.ptr.as_ptr()
        }

        pub fn put_u32(&mut self, attribute_id : i32, value : u32) -> i32 {
            unsafe {
                sys::nla_put_u32(self.ptr.as_ptr(), attribute_id, value)
            }
        }
    }



}
