use rustgym_msg::UserAgent;
use seed::{prelude::*, *};
use wasm_bindgen_test::*;

#[derive(Debug, Eq, PartialEq)]
pub enum DeviceHandler {
    Chrome74,
    Chrome70,
    Chrome67,
    Chrome55,
    Firefox60,
    Safari12,
    Safari11,
    Edge11,
}

pub trait DetectDevice {
    fn detect_device(&self) -> Option<DeviceHandler>;
}

impl DetectDevice for rustgym_msg::UserAgent {
    fn detect_device(&self) -> Option<DeviceHandler> {
        if self.family == "Chrome" {
            if let Some(major_str) = self.major.clone() {
                if let Ok(major_num) = major_str.parse::<i32>() {
                    if major_num >= 74 {
                        return Some(DeviceHandler::Chrome74);
                    }
                    if major_num >= 70 {
                        return Some(DeviceHandler::Chrome70);
                    }
                    if major_num >= 67 {
                        return Some(DeviceHandler::Chrome67);
                    }
                    if major_num >= 55 {
                        return Some(DeviceHandler::Chrome55);
                    }
                }
            }
        }
        if self.family == "Firefox" {
            if let Some(major_str) = self.major.clone() {
                if let Ok(major_num) = major_str.parse::<i32>() {
                    if major_num >= 60 {
                        return Some(DeviceHandler::Firefox60);
                    }
                }
            }
        }
        if self.family == "Safari" {
            if let Some(major_str) = self.major.clone() {
                if let Ok(major_num) = major_str.parse::<i32>() {
                    if major_num >= 12 {
                        return Some(DeviceHandler::Safari12);
                    }
                    if major_num >= 11 {
                        return Some(DeviceHandler::Safari11);
                    }
                }
            }
        }
        if self.family == "Edge11" {
            if let Some(major_str) = self.major.clone() {
                if let Ok(major_num) = major_str.parse::<i32>() {
                    if major_num >= 11 {
                        return Some(DeviceHandler::Edge11);
                    }
                }
            }
        }
        None
    }
}

#[wasm_bindgen_test]
fn detect_device_test() {
    let user_agent = UserAgent {
        family: "Chrome".to_string(),
        major: Some("91".to_string()),
        minor: Some("0".to_string()),
        patch: Some("4472".to_string()),
    };
    assert_eq!(user_agent.detect_device(), Some(DeviceHandler::Chrome74));
    let user_agent = UserAgent {
        family: "Firefox".to_string(),
        major: Some("85".to_string()),
        minor: Some("0".to_string()),
        patch: None,
    };
    assert_eq!(user_agent.detect_device(), Some(DeviceHandler::Firefox60));
    let user_agent = UserAgent {
        family: "Safari".to_string(),
        major: Some("14".to_string()),
        minor: Some("1".to_string()),
        patch: Some("1".to_string()),
    };
    assert_eq!(user_agent.detect_device(), Some(DeviceHandler::Safari12));
}
