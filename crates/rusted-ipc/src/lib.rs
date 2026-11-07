#![doc = "Inter-process communication contracts for Rusted OS."]

//! Transport-independent message contracts.
//!
//! This crate provides a minimal generic envelope: a [`MessageId`], a
//! [`MessageKind`], and a [`Message<T>`] struct with explicit request-response
//! correlation via `in_reply_to`.
//!
//! What this crate is *not*:
//! - No transport: no sockets, D-Bus, networking, or process spawning.
//! - No serialization: no Serde, no wire format, no version negotiation here.
//! - No runtime: no brokers, channels, queues, routers, or middleware.
//! - No async runtime, no threads, no global state, no ID generation.
//! - No timestamps, sender identities, or routing metadata. Those are added
//!   only with a demonstrated requirement.
//!
//! Future D-Bus or Tokio integration must build on top of these types without
//! changing their semantics.

/// Lightweight message identifier.
///
/// A transparent wrapper around `u64`. IDs are caller-assigned: there is no
/// random generation, no global counter, and no implicit allocation. Two
/// messages correlate when they share the expected identifier (see
/// [`Message::is_response_to`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageId(pub u64);

impl MessageId {
    /// Creates an identifier from an explicit value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Distinguishes the role of a [`Message`] in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKind {
    /// An initial request that may expect a response.
    Request,
    /// A reply that references its originating request.
    Response,
    /// A one-way notification that expects no reply.
    Event,
}

/// Generic, transport-independent message envelope.
///
/// `T` is the payload and can be any type; the envelope itself carries no
/// serialization or transport assumptions.
///
/// Correlation design: every message owns a unique [`MessageId`], and a
/// response additionally carries `in_reply_to` with the request identifier.
/// No separate correlation type is introduced because `Option<MessageId>` on
/// the same envelope expresses the relationship without a second abstraction
/// or a request-response runtime. Requests and events use `in_reply_to: None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message<T> {
    /// This message's own identifier.
    pub id: MessageId,
    /// Whether this message is a request, response, or event.
    pub kind: MessageKind,
    /// For responses, the identifier of the originating request.
    pub in_reply_to: Option<MessageId>,
    /// Caller-defined payload.
    pub payload: T,
}

impl<T> Message<T> {
    /// Builds a request with no correlation.
    pub const fn request(id: MessageId, payload: T) -> Self {
        Self {
            id,
            kind: MessageKind::Request,
            in_reply_to: None,
            payload,
        }
    }

    /// Builds a one-way event with no correlation.
    pub const fn event(id: MessageId, payload: T) -> Self {
        Self {
            id,
            kind: MessageKind::Event,
            in_reply_to: None,
            payload,
        }
    }

    /// Builds a response that references `request_id`.
    ///
    /// `id` should be this response's own fresh identifier, distinct from the
    /// request, so each message stays uniquely addressable while the link to
    /// the request is preserved in `in_reply_to`.
    pub const fn response(id: MessageId, request_id: MessageId, payload: T) -> Self {
        Self {
            id,
            kind: MessageKind::Response,
            in_reply_to: Some(request_id),
            payload,
        }
    }

    /// Returns true only for responses that reference `request_id`.
    ///
    /// Requests and events never correlate, even when identifiers match, so a
    /// stray event cannot be mistaken for a reply.
    pub const fn is_response_to(&self, request_id: MessageId) -> bool {
        match self.in_reply_to {
            Some(candidate) => {
                candidate.0 == request_id.0 && matches!(self.kind, MessageKind::Response)
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_id_equality_and_inequality() {
        assert_eq!(MessageId::new(7), MessageId::new(7));
        assert_ne!(MessageId::new(7), MessageId::new(8));
    }

    #[test]
    fn message_kind_equality() {
        assert_eq!(MessageKind::Request, MessageKind::Request);
        assert_ne!(MessageKind::Request, MessageKind::Response);
        assert_ne!(MessageKind::Response, MessageKind::Event);
    }

    #[test]
    fn request_message_construction() {
        let msg = Message::request(MessageId::new(1), "ping");

        assert_eq!(msg.id, MessageId::new(1));
        assert_eq!(msg.kind, MessageKind::Request);
        assert_eq!(msg.in_reply_to, None);
        assert_eq!(msg.payload, "ping");
        assert!(!msg.is_response_to(MessageId::new(1)));
    }

    #[test]
    fn response_message_construction() {
        let request_id = MessageId::new(10);
        let msg = Message::response(MessageId::new(11), request_id, "pong");

        assert_eq!(msg.id, MessageId::new(11));
        assert_eq!(msg.kind, MessageKind::Response);
        assert_eq!(msg.in_reply_to, Some(request_id));
        assert_eq!(msg.payload, "pong");
    }

    #[test]
    fn event_message_construction() {
        let msg = Message::event(MessageId::new(20), "changed");

        assert_eq!(msg.kind, MessageKind::Event);
        assert_eq!(msg.in_reply_to, None);
        assert!(!msg.is_response_to(MessageId::new(20)));
    }

    #[test]
    fn generic_payload_supports_different_types() {
        let int_msg = Message::request(MessageId::new(30), 42_i32);
        let owned_msg = Message::request(MessageId::new(31), String::from("data"));
        let tuple_msg = Message::event(MessageId::new(32), (true, 7_u8));

        assert_eq!(int_msg.payload, 42);
        assert_eq!(owned_msg.payload, "data");
        assert_eq!(tuple_msg.payload, (true, 7_u8));
        assert_eq!(tuple_msg.kind, MessageKind::Event);
    }

    #[test]
    fn request_response_correlation() {
        let request = Message::request(MessageId::new(40), "work");
        let response = Message::response(MessageId::new(41), request.id, "done");
        let unrelated = Message::response(MessageId::new(42), MessageId::new(99), "done");

        assert!(response.is_response_to(request.id));
        assert!(!unrelated.is_response_to(request.id));
        assert!(!request.is_response_to(request.id));
    }
}
