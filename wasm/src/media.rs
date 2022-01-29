use crate::utils::*;
use log::info;
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen::*;
use web_sys::{HtmlDivElement, HtmlVideoElement, MediaStream, MediaStreamTrack};

#[derive(Debug, Clone)]
pub struct MediaClient {
    local_video: HtmlVideoElement,
    remote_videos: HtmlDivElement,
    video_elements: HashMap<Uuid, HtmlDivElement>,
    media_streams: HashMap<Uuid, MediaStream>,
    video_tracks: HashMap<Uuid, MediaStreamTrack>,
    audio_tracks: HashMap<Uuid, MediaStreamTrack>,
}

impl MediaClient {
    pub fn new(local_video: HtmlVideoElement, remote_videos: HtmlDivElement) -> Self {
        let video_elements = HashMap::new();
        let video_tracks = HashMap::new();
        let audio_tracks = HashMap::new();
        let media_streams = HashMap::new();
        MediaClient {
            local_video,
            remote_videos,
            video_elements,
            media_streams,
            video_tracks,
            audio_tracks,
        }
    }

    pub fn init_local_video(&self, cam_media_stream: &MediaStream) -> Result<(), JsValue> {
        let video_only = MediaStream::new()?;
        let video_track: MediaStreamTrack =
            cam_media_stream.get_video_tracks().get(0).dyn_into()?;
        video_only.add_track(&video_track);
        self.local_video.set_src_object(Some(&video_only));
        Ok(())
    }

    pub fn add_remote_video(&mut self, remote: Uuid) -> Result<(), JsValue> {
        if self.video_elements.get(&remote).is_none() {
            let video = video();
            video.set_id(&remote.to_string());
            video.set_autoplay(true);
            let media_stream = MediaStream::new()?;
            if let Some(video_track) = self.video_tracks.get(&remote) {
                media_stream.add_track(video_track);
            }
            if let Some(audio_track) = self.audio_tracks.get(&remote) {
                media_stream.add_track(audio_track);
            }
            video.set_src_object(Some(&media_stream));
            let video_window = video_window(&remote.to_string(), &remote.to_string(), video);
            self.media_streams.insert(remote, media_stream);
            self.remote_videos.append_with_node_1(&video_window)?;
            self.video_elements.insert(remote, video_window);
        }
        Ok(())
    }

    pub fn remove_remote_video(&mut self, remote: Uuid) -> Result<(), JsValue> {
        self.video_tracks.remove(&remote);
        self.audio_tracks.remove(&remote);
        self.media_streams.remove(&remote);
        if let Some(el) = self.video_elements.remove(&remote) {
            self.remote_videos.remove_child(&el)?;
        } else {
            info!("else {}", remote);
        }
        Ok(())
    }

    pub fn add_remote_video_track(
        &mut self,
        remote: Uuid,
        video_track: MediaStreamTrack,
    ) -> Result<(), JsValue> {
        self.video_tracks.insert(remote, video_track);
        Ok(())
    }

    pub fn add_remote_audio_track(
        &mut self,
        remote: Uuid,
        audio_track: MediaStreamTrack,
    ) -> Result<(), JsValue> {
        self.audio_tracks.insert(remote, audio_track);
        Ok(())
    }
}
