use crate::desktop::Desktop;
use crate::media::MediaClient;
use crate::pc::PeerConnection;
use crate::searchbar::*;
use crate::utils::*;
use js_sys::JsString;
use js_sys::Reflect;
use log::info;
use rustgym_msg::*;
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::*;
use wasm_bindgen_futures::*;
use web_sys::{
    MediaStream, MediaStreamTrack, MessageEvent, RtcIceCandidate, RtcIceCandidateInit, RtcSdpType,
    RtcSessionDescriptionInit, WebSocket,
};

#[derive(Debug, Clone)]
pub struct Client {
    client_info: Option<ClientInfo>,
    ws: WebSocket,
    _desktop: Desktop,
    searchbar: Option<SearchBar>,
    media_client: Option<MediaClient>,
    media_stream: Option<MediaStream>,
    pcs: HashMap<Uuid, PeerConnection>,
}

impl Client {
    pub fn new() -> Self {
        let client_info = None;
        let media_client = None;
        let searchbar = None;
        let media_stream = None;
        let pcs = HashMap::new();
        let _desktop = Desktop::new(start_menu(), start_button());

        let ws = WebSocket::new(&wsurl()).expect("WebSocket");
        let onmessage_cb = Closure::wrap(Box::new(move |e: MessageEvent| {
            let js_json: JsString = e.data().dyn_into().unwrap();
            let rust_json: String = js_json.into();
            match serde_json::from_str::<MsgOut>(&rust_json) {
                Ok(msg) => {
                    process(msg);
                }
                Err(err) => {
                    info!("{}", err);
                }
            }
        }) as Box<dyn FnMut(_)>);
        ws.set_onmessage(Some(onmessage_cb.as_ref().unchecked_ref()));
        onmessage_cb.forget();

        Client {
            ws,
            client_info,
            media_client,
            media_stream,
            _desktop,
            searchbar,
            pcs,
        }
    }

    fn on_stream_start(&self, client_uuid: Uuid) -> Result<(), JsValue> {
        self.send_json(MsgIn::StreamStart { client_uuid })
    }

    fn on_offer(&self, caller: Uuid, callee: Uuid, offer_sdp: String) -> Result<(), JsValue> {
        self.send_json(MsgIn::Offer {
            caller,
            callee,
            offer_sdp,
        })
    }

    fn on_answer(&self, caller: Uuid, callee: Uuid, answer_sdp: String) -> Result<(), JsValue> {
        self.send_json(MsgIn::Answer {
            caller,
            callee,
            answer_sdp,
        })
    }

    pub fn on_ice_candidate(
        &self,
        local: Uuid,
        remote: Uuid,
        candidate: String,
        sdp_mid: String,
        sdp_m_line_index: u16,
    ) -> Result<(), JsValue> {
        self.send_json(MsgIn::IceCandidate {
            local,
            remote,
            candidate,
            sdp_mid,
            sdp_m_line_index,
        })
    }

    pub fn on_search_text_change(&self, search_text: String) -> Result<(), JsValue> {
        self.send_json(MsgIn::SearchText(search_text))
    }

    pub fn on_query_text(&self, query_text: String) -> Result<(), JsValue> {
        let searchbar = self.searchbar.as_ref().expect("searchbar");
        searchbar.update_search_input(&query_text);
        searchbar.close_search_suggestions()?;
        self.send_json(MsgIn::QueryText(query_text))?;
        Ok(())
    }

    fn send_json(&self, msg: MsgIn) -> Result<(), JsValue> {
        let str = serde_json::to_string(&msg).expect("to_string");
        self.ws.send_with_str(&str)
    }
}

pub fn process(message: MsgOut) {
    spawn_local(async move {
        use MsgOut::*;
        let res: Result<(), JsValue> = match message {
            Ping => Ok(()),
            Pong => Ok(()),
            SearchSuggestions(suggestions) => render_search_suggestions(suggestions),
            QueryResults(results) => render_query_results(results),
            RegistorClient(client_info) => process_register_client(client_info).await,
            UnRegistorClient(_) => Ok(()),
            StreamStart { client_uuid } => start_call(client_uuid).await,
            Offer {
                caller,
                callee,
                offer_sdp,
            } => process_offer(caller, callee, offer_sdp).await,
            Answer {
                caller,
                callee,
                answer_sdp,
            } => process_answer(caller, callee, answer_sdp).await,
            IceCandidate {
                local,
                remote,
                candidate,
                sdp_mid,
                sdp_m_line_index,
            } => process_ice_candidate(local, remote, candidate, sdp_mid, sdp_m_line_index).await,
            SessionClients(_) => Ok(()),
            AllClients(_) => Ok(()),
        };
        match res {
            Ok(_) => {}
            Err(err) => {
                info!("{:?}", err);
            }
        }
    });
}

async fn process_ice_candidate(
    _local: Uuid,
    remote: Uuid,
    candidate: String,
    sdp_mid: String,
    sdp_m_line_index: u16,
) -> Result<(), JsValue> {
    if let Some(pc) = get_client().pcs.get(&remote) {
        info!("ice {} {}", remote, candidate);
        let mut candidate_init = RtcIceCandidateInit::new(&candidate);
        candidate_init.sdp_mid(Some(&sdp_mid));
        candidate_init.sdp_m_line_index(Some(sdp_m_line_index));
        let candidate_obj = RtcIceCandidate::new(&candidate_init)?;
        let promise = pc.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&candidate_obj));
        JsFuture::from(promise).await?;
    }
    Ok(())
}

async fn process_offer(caller: Uuid, callee: Uuid, offer_sdp: String) -> Result<(), JsValue> {
    info!("process_offer");
    let media_stream = get_client().media_stream.expect("media_stream");
    let (pc, answer_sdp) = create_answer(caller, callee, &media_stream, offer_sdp).await?;
    set_peerconnection(caller, pc);
    get_client().on_answer(caller, callee, answer_sdp)?;
    Ok(())
}

async fn process_answer(_caller: Uuid, callee: Uuid, answer_sdp: String) -> Result<(), JsValue> {
    info!("process_answer");
    let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
    answer_obj.sdp(&answer_sdp);
    let srd_promise = get_client()
        .pcs
        .get(&callee)
        .expect("pc")
        .set_remote_description(&answer_obj);
    JsFuture::from(srd_promise).await?;
    Ok(())
}

async fn process_register_client(client_info: ClientInfo) -> Result<(), JsValue> {
    if get_client().client_info.is_none() {
        info!("set local {}", client_info.client_uuid);
        info!(
            "{:?}",
            client_info.user_agent.as_ref().expect("useragent").family
        );
        let client_uuid = client_info.client_uuid;
        if client_info.is_media_supported() {
            let media_stream = get_media_stream().await?;
            let media_client = get_client().media_client.expect("media_client");
            media_client.init_local_video(&media_stream)?;
            set_media_stream(media_stream);
            get_client().on_stream_start(client_uuid)?;
        }
        set_client_info(client_info);
    }
    Ok(())
}

async fn start_call(client_uuid: Uuid) -> Result<(), JsValue> {
    let local_client_info = get_client().client_info.expect("local_client_info");
    let caller = local_client_info.client_uuid;
    let callee = client_uuid;
    if callee != caller {
        info!("start_call");
        let (pc, offer_sdp) = create_offer(
            caller,
            callee,
            &get_client().media_stream.expect("media_stream"),
        )
        .await?;
        set_peerconnection(callee, pc);
        get_client().on_offer(caller, callee, offer_sdp)
    } else {
        Ok(())
    }
}

async fn create_offer(
    caller: Uuid,
    callee: Uuid,
    media_stream: &MediaStream,
) -> Result<(PeerConnection, String), JsValue> {
    let local_client_info = get_client().client_info.expect("local_client_info");
    let pc = PeerConnection::new(caller, callee, local_client_info.ice_servers)?;
    let tracks = media_stream.get_tracks().to_vec();
    for item in tracks {
        let media_stream_track: MediaStreamTrack = item.dyn_into().unwrap();
        pc.add_track_0(&media_stream_track, media_stream);
    }

    let offer = JsFuture::from(pc.create_offer()).await?;
    let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))?
        .as_string()
        .unwrap();
    let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    offer_obj.sdp(&offer_sdp);
    let sld_promise = pc.set_local_description(&offer_obj);
    JsFuture::from(sld_promise).await?;
    Ok((pc, offer_sdp))
}

async fn create_answer(
    caller: Uuid,
    callee: Uuid,
    media_stream: &MediaStream,
    offer_sdp: String,
) -> Result<(PeerConnection, String), JsValue> {
    info!("create_answer");
    let local_client_info = get_client().client_info.expect("local_client_info");
    let pc = PeerConnection::new(callee, caller, local_client_info.ice_servers)?;
    let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    offer_obj.sdp(&offer_sdp);
    let srd_promise = pc.set_remote_description(&offer_obj);
    JsFuture::from(srd_promise).await?;

    let tracks = media_stream.get_tracks().to_vec();
    for item in tracks {
        let media_stream_track: MediaStreamTrack = item.dyn_into().unwrap();
        pc.add_track_0(&media_stream_track, media_stream);
    }

    let answer = JsFuture::from(pc.create_answer()).await?;
    let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
    let answer_sdp = Reflect::get(&answer, &JsValue::from_str("sdp"))?
        .as_string()
        .unwrap();
    answer_obj.sdp(&answer_sdp);
    let sld_promise = pc.set_local_description(&answer_obj);
    JsFuture::from(sld_promise).await?;
    Ok((pc, answer_sdp))
}

fn render_search_suggestions(search_suggestions: Vec<String>) -> Result<(), JsValue> {
    info!("{:?}", search_suggestions);
    get_client()
        .searchbar
        .expect("searchbar")
        .update_search_suggestions(search_suggestions)
}

fn render_query_results(query_results: Vec<QueryResult>) -> Result<(), JsValue> {
    info!("{:?}", query_results);
    get_client()
        .searchbar
        .expect("searchbar")
        .update_query_results(query_results)
}

thread_local! {
    pub static CLIENT: RefCell<Client> = RefCell::new(Client::new());
}

pub fn get_client() -> Client {
    CLIENT.with(|client| client.borrow_mut().clone())
}

pub fn set_client_info(client_info: ClientInfo) {
    CLIENT.with(|client| client.borrow_mut().client_info = Some(client_info));
}

pub fn set_searchbar(searchbar: SearchBar) {
    CLIENT.with(|client| client.borrow_mut().searchbar = Some(searchbar));
}

pub fn set_media_client(media_client: MediaClient) {
    CLIENT.with(|client| client.borrow_mut().media_client = Some(media_client));
}

pub fn add_remote_video(remote: Uuid) {
    CLIENT.with(|client| {
        client
            .borrow_mut()
            .media_client
            .as_mut()
            .expect("media_client")
            .add_remote_video(remote)
            .expect("add_remote_video");
    });
}

pub fn remove_remote_video(remote: Uuid) {
    CLIENT.with(|client| {
        client
            .borrow_mut()
            .media_client
            .as_mut()
            .expect("media_client")
            .remove_remote_video(remote)
            .expect("remove_remote_video");
    });
}

pub fn add_remote_track(remote: Uuid, track: MediaStreamTrack) {
    CLIENT.with(|client| match track.kind().as_ref() {
        "video" => {
            client
                .borrow_mut()
                .media_client
                .as_mut()
                .expect("media_client")
                .add_remote_video_track(remote, track)
                .expect("add_remote_video_track");
        }
        "audio" => {
            client
                .borrow_mut()
                .media_client
                .as_mut()
                .expect("media_client")
                .add_remote_audio_track(remote, track)
                .expect("add_remote_audio_track");
        }
        _ => {
            info!("{:?} {}", track, track.kind());
        }
    });
}

fn set_media_stream(media_stream: MediaStream) {
    CLIENT.with(|client| client.borrow_mut().media_stream = Some(media_stream));
}

fn set_peerconnection(remote_uuid: Uuid, pc: PeerConnection) {
    CLIENT.with(|client| client.borrow_mut().pcs.insert(remote_uuid, pc));
}
