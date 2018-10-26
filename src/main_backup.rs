extern crate libc;

mod netlink;
mod taskstat;
pub const TASKSTATS_GENL_NAME: &'static [u8; 10usize] = b"TASKSTATS\0";

extern "C" fn message_handler(msg: *mut netlink::nl_msg, arg: *mut std::os::raw::c_void) -> std::os::raw::c_int
{
    use netlink::*;
    println!("hi im in the message handler");
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
            println!("got this far");
            let data=  nla_data(nla_next(nla_data(attr) as *const nlattr, &mut rem));
            let stats : &mut taskstat::taskstats = {
                &mut *(data as *mut taskstat::taskstats)
            };
            println!("stats: {:?}", stats);
        }
    }
    return 0
}

fn main() {
    let pid = std::process::id();
    let mut msg = unsafe { netlink::nlmsg_alloc() };
    println!("grabbing task stats for {}", pid);
    let sock = unsafe { netlink::nl_socket_alloc() };
    if sock.is_null() {
        panic!("failed to create netlink socket");
    }
    let err = unsafe { netlink::nl_connect(sock, libc::NETLINK_GENERIC) };
    if err < 0 {
        panic!("failed to connect to netlink socket");
    }
    let name = unsafe {
        TASKSTATS_GENL_NAME.as_ptr() as *const i8
    };
    let family = unsafe { netlink::genl_ctrl_resolve(sock, name) };
    if family == 0 {
        panic!("failed to get family id");
    }
    println!("family id is {}", family);
    let moderr = unsafe {
        netlink::nl_socket_modify_cb(
        sock,
        netlink::nl_cb_type_NL_CB_VALID,
        netlink::nl_cb_kind_NL_CB_CUSTOM,
        Some(message_handler),
        std::ptr::null_mut()
        )
    };
    if (moderr < 0) {
        println!("failed to setup callback");
    }

    unsafe {
        use netlink::*;
        genlmsg_put(msg, 0, 0, family, 0  , 1,  1, 1 );

        nla_put_u32(msg, taskstat::TASKSTATS_TYPE_PID as i32, pid);
        println!("sent message res: {}", nl_send_sync(sock, msg));
        nl_recvmsgs_default(sock);
    }

}
