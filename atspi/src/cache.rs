//! # `DBus` interface proxy for: `org.a11y.atspi.Cache`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Cache.xml`.
//!

use crate::atspi_proxy;
use atspi_common::CacheItem;
use zbus::zvariant::OwnedObjectPath;

#[atspi_proxy(interface = "org.a11y.atspi.Cache", default_path = "/org/a11y/atspi/cache")]
trait Cache {
	/// GetItems method
	fn get_items(&self) -> zbus::Result<Vec<CacheItem>>;

	/// AddAccessible signal
	#[dbus_proxy(signal)]
	fn add_accessible(&self, node_added: CacheItem) -> zbus::Result<()>;

	/// RemoveAccessible signal
	#[dbus_proxy(signal)]
	fn remove_accessible(&self, node_removed: (String, OwnedObjectPath)) -> zbus::Result<()>;
}
