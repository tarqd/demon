#[macro_use]
extern crate num_derive;
extern crate num_traits;
extern crate nix;

extern crate libc;

mod netlink;
mod taskstat;
use num_traits::ToPrimitive;
pub const TASKSTATS_GENL_NAME: &'static [u8; 10usize] = b"TASKSTATS\0";

extern "C" fn message_handler(msg: *mut netlink::sys::nl_msg, arg: *mut std::os::raw::c_void) -> std::os::raw::c_int
{
    use netlink::*;
    use netlink::sys::*;
    unsafe {
        let mut attrs : [*mut nlattr; 7 + 1] = std::mem::uninitialized();
        let ptr = attrs.as_mut_ptr();
        let hdr = nlmsg_hdr(msg);
//        let stats : *mut taskstat::taskstats = std::ptr::null_mut();
        let mut attr: *const nlattr = std::ptr::null();
        let mut rem = 0;
        let answer = genlmsg_parse(hdr, 0, ptr, 7,
                                   std::ptr::null_mut());
        if !attrs[taskstat::TASKSTATS_TYPE_AGGR_PID as usize].is_null() {
            attr = attrs[taskstat::TASKSTATS_TYPE_AGGR_PID as usize];
            let data=  nla_data(nla_next(nla_data(attr) as *const nlattr, &mut rem));
            let stats : &mut taskstat::taskstats = {
                &mut *(data as *mut taskstat::taskstats)
            };
            println!("steal% over proc lifetime: {:?}", stats.cpu_delay_total / stats.cpu_run_real_total);
        }
    }
    return 0
}
fn stat_message_for_pid(family : i32, pid: u32) -> netlink::message::Message {
    use netlink::message::*;
    let mut msg = Message::new();
    use netlink::sys::*;
    unsafe {
        genlmsg_put(msg.as_ptr(), 0, 0, family, 0, 1, 1, 1);
        nla_put_u32(msg.as_ptr(), taskstat::TASKSTATS_TYPE_PID as i32, pid);
    }
    msg
}

fn get_family(sock : &mut netlink::socket::Socket) -> i32 {
    let name = unsafe {
        TASKSTATS_GENL_NAME.as_ptr() as *const i8
    };
    unsafe { netlink::sys::genl_ctrl_resolve(sock.as_ptr(), name) }
}

fn main() {
    let pid = std::process::id();
    let mut sock = netlink::socket::Socket::new();

    sock.connect(netlink::socket::Family::Generic).unwrap();
    let family = get_family(&mut sock);
    if family == 0 {
        panic!("failed to get family id");
    }

    println!("family id is {}", family);
    let moderr = unsafe {
        netlink::sys::nl_socket_modify_cb(
            sock.as_ptr(),
            netlink::sys::nl_cb_type_NL_CB_VALID,
            netlink::sys::nl_cb_kind_NL_CB_CUSTOM,
            Some(message_handler),
            std::ptr::null_mut()
        )
    };
    if (moderr < 0) {
        println!("failed to setup callback");
    }

    unsafe {
        use netlink::sys::*;
        loop {
            let mut msg = stat_message_for_pid(family, pid);
            nl_send_sync(sock.as_ptr(), msg.as_ptr());
            nl_recvmsgs_default(sock.as_ptr());
            std::thread::sleep_ms(1000);
        }
    }

}

