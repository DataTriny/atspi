//! # `DBus` interface proxy for: `org.a11y.atspi.Table`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Table.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use crate::atspi_proxy;
use atspi_common::Accessible;

#[atspi_proxy(interface = "org.a11y.atspi.Table", assume_defaults = true)]
trait Table {
	/// AddColumnSelection method
	fn add_column_selection(&self, column: i32) -> zbus::Result<bool>;

	/// AddRowSelection method
	fn add_row_selection(&self, row: i32) -> zbus::Result<bool>;

	/// GetAccessibleAt method
	fn get_accessible_at(
		&self,
		row: i32,
		column: i32,
	) -> zbus::Result<Accessible>;

	/// GetColumnAtIndex method
	fn get_column_at_index(&self, index: i32) -> zbus::Result<i32>;

	/// GetColumnDescription method
	fn get_column_description(&self, column: i32) -> zbus::Result<String>;

	/// GetColumnExtentAt method
	fn get_column_extent_at(&self, row: i32, column: i32) -> zbus::Result<i32>;

	/// GetColumnHeader method
	fn get_column_header(
		&self,
		column: i32,
	) -> zbus::Result<Accessible>;

	/// GetIndexAt method
	fn get_index_at(&self, row: i32, column: i32) -> zbus::Result<i32>;

	/// GetRowAtIndex method
	fn get_row_at_index(&self, index: i32) -> zbus::Result<i32>;

	/// GetRowColumnExtentsAtIndex method
	fn get_row_column_extents_at_index(
		&self,
		index: i32,
	) -> zbus::Result<(bool, i32, i32, i32, i32, bool)>;

	/// GetRowDescription method
	fn get_row_description(&self, row: i32) -> zbus::Result<String>;

	/// GetRowExtentAt method
	fn get_row_extent_at(&self, row: i32, column: i32) -> zbus::Result<i32>;

	/// GetRowHeader method
	fn get_row_header(&self, row: i32) -> zbus::Result<Accessible>;

	/// GetSelectedColumns method
	fn get_selected_columns(&self) -> zbus::Result<Vec<i32>>;

	/// GetSelectedRows method
	fn get_selected_rows(&self) -> zbus::Result<Vec<i32>>;

	/// IsColumnSelected method
	fn is_column_selected(&self, column: i32) -> zbus::Result<bool>;

	/// IsRowSelected method
	fn is_row_selected(&self, row: i32) -> zbus::Result<bool>;

	/// IsSelected method
	fn is_selected(&self, row: i32, column: i32) -> zbus::Result<bool>;

	/// RemoveColumnSelection method
	fn remove_column_selection(&self, column: i32) -> zbus::Result<bool>;

	/// RemoveRowSelection method
	fn remove_row_selection(&self, row: i32) -> zbus::Result<bool>;

	/// Caption property
	#[dbus_proxy(property)]
	fn caption(&self) -> zbus::Result<Accessible>;

	/// NColumns property
	#[dbus_proxy(property)]
	fn ncolumns(&self) -> zbus::Result<i32>;

	/// NRows property
	#[dbus_proxy(property)]
	fn nrows(&self) -> zbus::Result<i32>;

	/// NSelectedColumns property
	#[dbus_proxy(property)]
	fn nselected_columns(&self) -> zbus::Result<i32>;

	/// NSelectedRows property
	#[dbus_proxy(property)]
	fn nselected_rows(&self) -> zbus::Result<i32>;

	/// Summary property
	#[dbus_proxy(property)]
	fn summary(&self) -> zbus::Result<Accessible>;
}
