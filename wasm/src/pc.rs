use crate::client::*;
use log::info;
use rustgym_msg::*;
use std::ops::Deref;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use web_sys::{
    Event, RtcConfiguration, RtcIceConnectionState, RtcPeerConnection, RtcPeerConnectionIceEvent,
    RtcTrackEvent,
};

#[derive(Debug, Clone)]
pub struct PeerConnection {
    _local: Uuid,
    _remote: Uuid,
    pc: RtcPeerConnection,
}

impl PeerConnection {
    pub fn new(local: Uuid, remote: Uuid, ice_servers: Vec<IceServer>) -> Result<Self, JsValue> {
        let mut config = RtcConfiguration::new();
        info!("{:?}", ice_servers);
        let ice_servers = JsValue::from_serde(&ice_servers)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        config.ice_servers(&ice_servers);
        let pc = RtcPeerConnection::new_with_configuration(&config)?;
        let onicecandidate_cb = Closure::wrap(Box::new(move |e: RtcPeerConnectionIceEvent| {
            if let Some(candidate_obj) = e.candidate() {
                let candidate: String = candidate_obj.candidate();
                info!("{}", candidate);
                let sdp_mid: String = candidate_obj.sdp_mid().expect("sdp_mid");
                let sdp_m_line_index: u16 =
                    candidate_obj.sdp_m_line_index().expect("sdp_m_line_index");
                get_client()
                    .on_ice_candidate(local, remote, candidate, sdp_mid, sdp_m_line_index)
                    .expect("on_ice_candidate");
            }
        }) as Box<dyn FnMut(_)>);
        pc.set_onicecandidate(Some(onicecandidate_cb.as_ref().unchecked_ref()));
        onicecandidate_cb.forget();

        let pc_clone = pc.clone();
        let oniceconnectionstatechange_cb = Closure::wrap(Box::new(move |_e: Event| {
            use RtcIceConnectionState::*;
            let state = match pc_clone.ice_connection_state() {
                New => "new",
                Checking => "checking",
                Connected => {
                    add_remote_video(remote);
                    "connected"
                }
                Completed => "completed",
                Failed => "failed",
                Disconnected => {
                    remove_remote_video(remote);
                    "disconnected"
                }
                Closed => "closed",
                _ => "",
            };
            info!("{} {:?}", remote, state);
        }) as Box<dyn FnMut(_)>);
        pc.set_oniceconnectionstatechange(Some(
            oniceconnectionstatechange_cb.as_ref().unchecked_ref(),
        ));
        oniceconnectionstatechange_cb.forget();

        let pc_clone = pc.clone();
        let onicegatheringstatechange_cb = Closure::wrap(Box::new(move |_e: Event| {
            info!("{:?}", pc_clone.ice_gathering_state());
        }) as Box<dyn FnMut(_)>);
        pc.set_onicegatheringstatechange(Some(
            onicegatheringstatechange_cb.as_ref().unchecked_ref(),
        ));
        onicegatheringstatechange_cb.forget();

        let pc_clone = pc.clone();
        let onsignalingstatechange_cb = Closure::wrap(Box::new(move |_e: Event| {
            info!("{:?}", pc_clone.signaling_state());
        }) as Box<dyn FnMut(_)>);
        pc.set_onsignalingstatechange(Some(onsignalingstatechange_cb.as_ref().unchecked_ref()));
        onsignalingstatechange_cb.forget();

        let pc_clone = pc.clone();
        let onnegotiationneeded_cb = Closure::wrap(Box::new(move |_e: Event| {
            info!("{:?}", pc_clone.signaling_state());
        }) as Box<dyn FnMut(_)>);
        pc.set_onnegotiationneeded(Some(onnegotiationneeded_cb.as_ref().unchecked_ref()));
        onnegotiationneeded_cb.forget();

        let ontrack_cb = Closure::wrap(Box::new(move |e: RtcTrackEvent| {
            let track = e.track();
            info!("{:?} {}", track, track.kind());
            add_remote_track(remote, track);
        }) as Box<dyn FnMut(_)>);
        pc.set_ontrack(Some(ontrack_cb.as_ref().unchecked_ref()));
        ontrack_cb.forget();

        Ok(PeerConnection {
            _local: local,
            _remote: remote,
            pc,
        })
    }
}

impl Deref for PeerConnection {
    type Target = RtcPeerConnection;

    fn deref(&self) -> &Self::Target {
        &self.pc
    }
}
