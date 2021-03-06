//! # DBus interface proxy for: `org.opensuse.DInstaller.Software1`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/opensuse/DInstaller/Software1' from service 'org.opensuse.DInstaller.Software' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(
    default_service = "org.opensuse.DInstaller.Software",
    interface = "org.opensuse.DInstaller.Software1",
    default_path = "/org/opensuse/DInstaller/Software1",
    gen_async = false,
    gen_blocking = true
)]
trait Software1 {
    /// Finish method
    fn finish(&self) -> zbus::Result<()>;

    /// Install method
    fn install(&self) -> zbus::Result<()>;

    /// Probe method
    fn probe(&self) -> zbus::Result<()>;

    /// Propose method
    fn propose(&self) -> zbus::Result<()>;

    /// ProvisionSelected method
    #[dbus_proxy(name = "Provision")]
    fn provision_selected(&self, provision: &str) -> zbus::Result<bool>;

    /// ProvisionsSelected method
    #[dbus_proxy(name = "Provisions")]
    fn provisions_selected(&self, provisions: &[&str]) -> zbus::Result<Vec<bool>>;

    /// SelectProduct method
    #[dbus_proxy(name = "SelectProduct")]
    fn select_product(&self, product_id: &str) -> zbus::Result<()>;

    /// AvailableBaseProducts property
    #[dbus_proxy(property)]
    fn available_base_products(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// Progress property
    #[dbus_proxy(property)]
    fn progress(&self) -> zbus::Result<(String, u64, u64, u64, u64)>;

    /// SelectedBaseProduct property
    #[dbus_proxy(property)]
    fn selected_base_product(&self) -> zbus::Result<String>;

    /// Status property
    #[dbus_proxy(property)]
    fn status(&self) -> zbus::Result<u32>;
}
