//! # DBus interface proxy for: `org.opensuse.DInstaller.Language1`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/opensuse/DInstaller/Language1' from service 'org.opensuse.DInstaller' on system bus`.
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
    default_service = "org.opensuse.DInstaller.Language",
    interface = "org.opensuse.DInstaller.Language1",
    default_path = "/org/opensuse/DInstaller/Language1",
    gen_async = false,
    gen_blocking = true
)]
trait Language1 {
    /// ToInstall method
    fn to_install(&self, lang_ids: &[&str]) -> zbus::Result<()>;

    /// AvailableLanguages property
    #[dbus_proxy(property)]
    fn available_languages(
        &self,
    ) -> zbus::Result<
        Vec<(
            String,
            String,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        )>,
    >;

    /// MarkedForInstall property
    #[dbus_proxy(property)]
    fn marked_for_install(&self) -> zbus::Result<Vec<String>>;
}
