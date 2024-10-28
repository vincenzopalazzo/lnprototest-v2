use std::cell::RefCell;
use std::sync::Arc;
use std::time::SystemTime;

use lampo_common::bitcoin::constants::ChainHash;
use lampo_common::bitcoin::secp256k1;
use lampo_common::bitcoin::secp256k1::PublicKey;
use lampo_common::error;
use lampo_common::ldk::events::{MessageSendEvent, MessageSendEventsProvider};
use lampo_common::ldk::ln::msgs::ChannelMessageHandler;
use lampo_common::ldk::ln::peer_handler::{IgnoringMessageHandler, MessageHandler};
use lampo_common::ldk::ln::peer_handler::{SimpleArcPeerManager, SocketDescriptor};
use lampo_common::ldk::sign::InMemorySigner;
use lampo_common::ldk::util::ser::Writeable;
use lampo_common::utils::logger::LampoLogger;

pub struct Socket<S: SocketDescriptor> {
    descriptor: S,
    inner: SimpleArcPeerManager<
        S,
        InMemorySigner,
        Arc<InnerSocket>,
        IgnoringMessageHandler,
        IgnoringMessageHandler,
        LampoLogger,
    >,
}

impl<S: SocketDescriptor> Socket<S> {
    pub fn new(descriptor: &S, logger: Arc<LampoLogger>) -> Self {
        let inner = InnerSocket {
            queue: RefCell::new(Vec::new()),
        };

        let ephemeral_bytes = [0; 32];
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let lightning_msg_handler = MessageHandler {
            chan_handler: Arc::new(inner),
            onion_message_handler: IgnoringMessageHandler {},
            route_handler: IgnoringMessageHandler {},
            custom_message_handler: IgnoringMessageHandler {},
        };

        let inner = SimpleArcPeerManager::new(
            lightning_msg_handler,
            current_time.try_into().unwrap(),
            &ephemeral_bytes,
            logger,
        );
        Socket {
            descriptor: descriptor.clone(),
            inner,
        }
    }

    pub fn connect(&self, addr: &str) -> error::Result<()> {
        Ok(())
    }

    /// Send the data to a specific peer (we may need to specify the descriptor somehow)
    pub fn send(&mut self, data: &[u8]) -> error::Result<usize> {
        let size = self.descriptor.send_data(data, false);
        Ok(size)
    }

    pub fn recv(&self, data: &mut [u8]) -> error::Result<usize> {
        Ok(0)
    }

    pub fn close(&mut self) -> error::Result<()> {
        self.descriptor.disconnect_socket();
        Ok(())
    }
}

struct InnerSocket {
    queue: RefCell<Vec<Vec<u8>>>,
}

impl InnerSocket {
    pub fn add_msg<W: Writeable>(&self, source: PublicKey, msg: W) {
        let mut msgs = vec![msg.encode()];
        let mut old_msgs = self.queue.borrow_mut();
        msgs.append(&mut old_msgs);
    }
}

impl MessageSendEventsProvider for InnerSocket {
    fn get_and_clear_pending_msg_events(&self) -> Vec<MessageSendEvent> {
        Vec::new()
    }
}

impl ChannelMessageHandler for InnerSocket {
    fn get_chain_hashes(&self) -> Option<Vec<ChainHash>> {
        None
    }

    fn handle_accept_channel(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::AcceptChannel,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_accept_channel_v2(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::AcceptChannelV2,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_announcement_signatures(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::AnnouncementSignatures,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_channel_ready(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::ChannelReady,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_channel_reestablish(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::ChannelReestablish,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_channel_update(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::ChannelUpdate,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_closing_signed(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::ClosingSigned,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_commitment_signed(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::CommitmentSigned,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_error(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::ErrorMessage,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_funding_created(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::FundingCreated,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_funding_signed(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::FundingSigned,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_open_channel(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::OpenChannel,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_open_channel_v2(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::OpenChannelV2,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_revoke_and_ack(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::RevokeAndACK,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_shutdown(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::Shutdown,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_stfu(&self, their_node_id: &PublicKey, msg: &lampo_common::ldk::ln::msgs::Stfu) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_abort(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxAbort,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_ack_rbf(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxAckRbf,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_add_input(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxAddInput,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_add_output(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxAddOutput,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_complete(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxComplete,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_init_rbf(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxInitRbf,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_remove_input(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxRemoveInput,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_remove_output(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxRemoveOutput,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_tx_signatures(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::TxSignatures,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_update_add_htlc(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::UpdateAddHTLC,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_update_fail_htlc(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::UpdateFailHTLC,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_update_fail_malformed_htlc(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::UpdateFailMalformedHTLC,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_update_fee(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::UpdateFee,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn handle_update_fulfill_htlc(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::UpdateFulfillHTLC,
    ) {
        self.add_msg(*their_node_id, msg);
    }

    fn peer_connected(
        &self,
        their_node_id: &PublicKey,
        msg: &lampo_common::ldk::ln::msgs::Init,
        inbound: bool,
    ) -> Result<(), ()> {
        self.add_msg(*their_node_id, msg);
        Ok(())
    }

    fn peer_disconnected(&self, their_node_id: &PublicKey) {
        log::info!("Peer disconnected: `{}`", their_node_id);
    }

    fn provided_init_features(
        &self,
        their_node_id: &PublicKey,
    ) -> lampo_common::ldk::ln::features::InitFeatures {
        lampo_common::ldk::ln::features::InitFeatures::empty()
    }

    fn provided_node_features(&self) -> lampo_common::ldk::ln::features::NodeFeatures {
        lampo_common::ldk::ln::features::NodeFeatures::empty()
    }
}
