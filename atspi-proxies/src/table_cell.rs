//! # `DBus` interface proxy for: `org.a11y.atspi.TableCell`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `TableCell.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use crate::atspi_proxy;
use atspi_common::Accessible;

#[atspi_proxy(interface = "org.a11y.atspi.TableCell", assume_defaults = true)]
trait TableCell {
	/// GetColumnHeaderCells method
	fn get_column_header_cells(&self) -> zbus::Result<Vec<Accessible>>;

	/// GetRowColumnSpan method
	fn get_row_column_span(&self) -> zbus::Result<(bool, i32, i32, i32, i32)>;

	/// GetRowHeaderCells method
	fn get_row_header_cells(&self) -> zbus::Result<Vec<Accessible>>;

	/// ColumnSpan property
	#[dbus_proxy(property)]
	fn column_span(&self) -> zbus::Result<i32>;

	/// Position property
	#[dbus_proxy(property)]
	fn position(&self) -> zbus::Result<(i32, i32)>;

	/// RowSpan property
	#[dbus_proxy(property)]
	fn row_span(&self) -> zbus::Result<i32>;

	/// Table property
	#[dbus_proxy(property)]
	fn table(&self) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath)>;
}
