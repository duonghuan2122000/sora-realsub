use super::error::CoreError;
use cpal::traits::{DeviceTrait, HostTrait};
use serde::Serialize;

/// Thông tin một thiết bị audio input (mic) để UI hiển thị cho người dùng chọn.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AudioDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

/// Trừu tượng hoá nguồn liệt kê thiết bị audio của hệ điều hành, để test bằng mock
/// không cần phần cứng thật.
pub trait AudioHost {
    fn input_devices(&self) -> Result<Vec<AudioDeviceInfo>, CoreError>;
    fn output_devices(&self) -> Result<Vec<AudioDeviceInfo>, CoreError>;
}

/// Cài đặt thật dùng `cpal` (WASAPI trên Windows).
pub struct CpalAudioHost;

fn collect_devices(
    devices: impl Iterator<Item = cpal::Device>,
    default_name: Option<String>,
) -> Vec<AudioDeviceInfo> {
    let mut result = Vec::new();
    for device in devices {
        let name = device.name().unwrap_or_default();
        let display_name = if name.trim().is_empty() {
            "Thiết bị không tên".to_string()
        } else {
            name.clone()
        };
        let is_default = default_name.as_deref() == Some(name.as_str());
        result.push(AudioDeviceInfo {
            id: name,
            name: display_name,
            is_default,
        });
    }
    result
}

impl AudioHost for CpalAudioHost {
    fn input_devices(&self) -> Result<Vec<AudioDeviceInfo>, CoreError> {
        let host = cpal::default_host();
        let default_name = host.default_input_device().and_then(|d| d.name().ok());
        let devices = host
            .input_devices()
            .map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(collect_devices(devices, default_name))
    }

    fn output_devices(&self) -> Result<Vec<AudioDeviceInfo>, CoreError> {
        let host = cpal::default_host();
        let default_name = host.default_output_device().and_then(|d| d.name().ok());
        let devices = host
            .output_devices()
            .map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(collect_devices(devices, default_name))
    }
}

/// Liệt kê thiết bị audio input qua một `AudioHost` bất kỳ (thật hoặc mock).
pub fn list_input_devices(host: &dyn AudioHost) -> Result<Vec<AudioDeviceInfo>, CoreError> {
    host.input_devices()
}

/// Liệt kê thiết bị audio output (dùng làm nguồn loopback để chọn) qua một `AudioHost` bất kỳ.
pub fn list_output_devices(host: &dyn AudioHost) -> Result<Vec<AudioDeviceInfo>, CoreError> {
    host.output_devices()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockHost(Result<Vec<AudioDeviceInfo>, CoreError>);

    impl AudioHost for MockHost {
        fn input_devices(&self) -> Result<Vec<AudioDeviceInfo>, CoreError> {
            match &self.0 {
                Ok(v) => Ok(v.clone()),
                Err(_) => Err(CoreError::Io("lỗi truy vấn thiết bị".to_string())),
            }
        }

        fn output_devices(&self) -> Result<Vec<AudioDeviceInfo>, CoreError> {
            self.input_devices()
        }
    }

    #[test]
    fn co_thiet_bi_tra_dung_danh_sach_va_danh_dau_mac_dinh() {
        let host = MockHost(Ok(vec![
            AudioDeviceInfo {
                id: "Mic A".into(),
                name: "Mic A".into(),
                is_default: false,
            },
            AudioDeviceInfo {
                id: "Mic B".into(),
                name: "Mic B".into(),
                is_default: true,
            },
        ]));

        let result = list_input_devices(&host).expect("phải thành công");
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|d| d.name == "Mic B" && d.is_default));
        assert!(result.iter().any(|d| d.name == "Mic A" && !d.is_default));
    }

    #[test]
    fn khong_co_thiet_bi_tra_danh_sach_rong() {
        let host = MockHost(Ok(vec![]));
        let result = list_input_devices(&host).expect("phải thành công");
        assert!(result.is_empty());
    }

    #[test]
    fn thiet_bi_ten_rong_hien_thi_ten_thay_the() {
        let host = MockHost(Ok(vec![AudioDeviceInfo {
            id: "".into(),
            name: "Thiết bị không tên".into(),
            is_default: false,
        }]));
        let result = list_input_devices(&host).expect("phải thành công");
        assert_eq!(result[0].name, "Thiết bị không tên");
    }

    #[test]
    fn host_loi_tra_ve_coreerror_ro_nghia() {
        let host = MockHost(Err(CoreError::Io("x".into())));
        let err = list_input_devices(&host).expect_err("phải lỗi");
        assert!(matches!(err, CoreError::Io(_)));
    }

    #[test]
    fn list_output_devices_tra_dung_danh_sach() {
        let host = MockHost(Ok(vec![AudioDeviceInfo {
            id: "Speaker A".into(),
            name: "Speaker A".into(),
            is_default: true,
        }]));
        let result = list_output_devices(&host).expect("phải thành công");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Speaker A");
    }

    #[test]
    fn list_output_devices_rong_tra_danh_sach_rong() {
        let host = MockHost(Ok(vec![]));
        let result = list_output_devices(&host).expect("phải thành công");
        assert!(result.is_empty());
    }

    #[test]
    fn list_output_devices_loi_tra_ve_coreerror() {
        let host = MockHost(Err(CoreError::Io("x".into())));
        let err = list_output_devices(&host).expect_err("phải lỗi");
        assert!(matches!(err, CoreError::Io(_)));
    }
}
