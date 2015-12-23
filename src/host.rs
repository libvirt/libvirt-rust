use error;

#[allow(non_camel_case_types)]
#[repr(C)]
#[warn(improper_ctypes)]
struct virConnect;

#[link(name = "virt")]
extern {
    fn virConnectOpen(name: *const u8) -> *mut virConnect;
    fn virConnectClose(conn: *const virConnect) -> *const u8;
}

pub struct Host {
    conn: *mut virConnect
}

impl Host {
    /// This function should be called first to get a connection to
    /// the Hypervisor and xen store.
    ///
    /// If @name is NULL, if the LIBVIRT_DEFAULT_URI environment
    /// variable is set, then it will be used. Otherwise if the client
    /// configuration file has the "uri_default" parameter set, then
    /// it will be used. Finally probing will be done to determine a
    /// suitable default driver to activate. This involves trying each
    /// hypervisor in turn until one successfully opens.
    ///
    /// If connecting to an unprivileged hypervisor driver which
    /// requires the libvirtd daemon to be active, it will
    /// automatically be launched if not already running. This can be
    /// prevented by setting the environment variable
    /// LIBVIRT_AUTOSTART=0
    ///
    /// URIs are documented at http://libvirt.org/uri.html
    ///
    /// Host.close should be used to release the resources after the
    /// connection is no longer needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use virt::host::Host;
    ///
    /// match Host:: open("test:///default") {
    ///   Ok(h) => {
    ///     println!("Hello libvirt!");
    ///     h.close()
    ///   }
    ///   Err(e) => println!(
    ///     "failed with code {}, message: {}", e.code, e.message)
    /// }
    /// ```
    pub fn open(uri: &str) -> Result<Host, error::VirtError> {
        unsafe {
            let conn = virConnectOpen(uri.as_ptr());
            if conn.is_null() {
                return Err(error::VirtError::get_last_error());
            }
            Ok(Host{conn: conn})
        }
    }

    pub fn close(&self) {
        unsafe {
            virConnectClose(self.conn);
        }
    }
}


#[test]
fn it_works() {
}
