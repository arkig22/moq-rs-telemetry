use crate::coding::{DecodeError, EncodeError, VarInt};

use crate::coding::{AsyncRead, AsyncWrite};

use std::ops::Deref;

/// A version number negotiated during the setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(pub VarInt);

impl Version {
	/// <https://www.ietf.org/archive/id/draft-ietf-moq-transport-00.html>
	pub const DRAFT_00: Version = Version(VarInt::from_u32(0xff00));

	/// Fork of draft-ietf-moq-transport-00.
	///
	/// Rough list of differences:
	///
	/// # Messages
	/// - Messages are sent over a control stream or a data stream.
	///   - Data streams: each unidirectional stream contains a single OBJECT message.
	///   - Control stream: a (client-initiated) bidirectional stream containing SETUP and then all other messages.
	/// - Messages do not contain a length; unknown messages are fatal.
	///
	/// # SETUP
	/// - SETUP is split into SETUP_CLIENT and SETUP_SERVER with separate IDs.
	/// - SETUP uses version `0xff00` for draft-00.
	/// - SETUP no longer contains optional parameters; all are encoded in order and possibly zero.
	/// - SETUP `role` indicates the role of the sender, not the role of the server.
	/// - SETUP `path` field removed; use WebTransport for path.
	///
	/// # SUBSCRIBE
	/// - SUBSCRIBE `full_name` is split into separate `namespace` and `name` fields.
	/// - SUBSCRIBE no longer contains optional parameters; all are encoded in order and possibly zero.
	/// - SUBSCRIBE no longer contains the `auth` parameter; use WebTransport for auth.
	/// - SUBSCRIBE no longer contains the `group` parameter; concept no longer exists.
	/// - SUBSCRIBE contains the `id` instead of SUBSCRIBE_OK.
	/// - SUBSCRIBE_OK and SUBSCRIBE_ERROR reference the subscription `id` the instead of the track `full_name`.
	/// - SUBSCRIBE_ERROR was renamed to SUBSCRIBE_RESET, sent by publisher to terminate a SUBSCRIBE.
	/// - SUBSCRIBE_STOP was added, sent by the subscriber to terminate a SUBSCRIBE.
	/// - SUBSCRIBE_OK no longer has `expires`.
	///
	/// # ANNOUNCE
	/// - ANNOUNCE no longer contains optional parameters; all are encoded in order and possibly zero.
	/// - ANNOUNCE no longer contains the `auth` field; use WebTransport for auth.
	/// - ANNOUNCE_ERROR was renamed to ANNOUNCE_RESET, sent by publisher to terminate an ANNOUNCE.
	/// - ANNOUNCE_STOP was added, sent by the subscriber to terminate an ANNOUNCE.
	///
	/// # OBJECT
	/// - OBJECT uses a dedicated QUIC stream.
	/// - OBJECT has no size and continues until stream FIN.
	/// - OBJECT `priority` is a i32 instead of a varint. (for practical reasons)
	/// - OBJECT `expires` was added, a varint in seconds.
	/// - OBJECT `group` was removed.
	///
	/// # GROUP
	/// - GROUP concept was removed, replaced with OBJECT as a QUIC stream.
	pub const KIXEL_00: Version = Version(VarInt::from_u32(0xbad00));
}

impl From<VarInt> for Version {
	fn from(v: VarInt) -> Self {
		Self(v)
	}
}

impl From<Version> for VarInt {
	fn from(v: Version) -> Self {
		v.0
	}
}

impl Version {
	/// Decode the version number.
	pub async fn decode<R: AsyncRead>(r: &mut R) -> Result<Self, DecodeError> {
		let v = VarInt::decode(r).await?;
		Ok(Self(v))
	}

	/// Encode the version number.
	pub async fn encode<W: AsyncWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
		self.0.encode(w).await?;
		Ok(())
	}
}

/// A list of versions in arbitrary order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Versions(Vec<Version>);

impl Versions {
	/// Decode the version list.
	pub async fn decode<R: AsyncRead>(r: &mut R) -> Result<Self, DecodeError> {
		let count = VarInt::decode(r).await?.into_inner();
		let mut vs = Vec::new();

		for _ in 0..count {
			let v = Version::decode(r).await?;
			vs.push(v);
		}

		Ok(Self(vs))
	}

	/// Encode the version list.
	pub async fn encode<W: AsyncWrite>(&self, w: &mut W) -> Result<(), EncodeError> {
		let size: VarInt = self.0.len().try_into()?;
		size.encode(w).await?;

		for v in &self.0 {
			v.encode(w).await?;
		}

		Ok(())
	}
}

impl Deref for Versions {
	type Target = Vec<Version>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<Vec<Version>> for Versions {
	fn from(vs: Vec<Version>) -> Self {
		Self(vs)
	}
}
