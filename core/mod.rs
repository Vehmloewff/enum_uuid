pub use derive_enum_uuid::EnumUuid;
use uuid::Uuid;

pub trait EnumUuid: Sized {
	fn from_id(id: Uuid) -> Option<Self>;
	fn to_id(self) -> Uuid;
}
