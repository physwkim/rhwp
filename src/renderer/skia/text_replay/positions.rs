// 화면 원점과 글줄 안의 상대 위치를 f64로 합산한 뒤 Skia 좌표로 변환한다.
// 별도 변환 후 합산하면 같은 글자가 런 경계에 따라 다른 픽셀에 놓일 수 있다.
pub(super) fn device_x(origin: f64, offset: f64) -> f32 {
    (origin + offset) as f32
}

#[cfg(test)]
mod tests {
    use super::device_x;

    #[test]
    fn run_split_preserves_device_position_without_system_fonts() {
        let position = 195.7333333333334_f64;
        let whole_origin = 123.36000000000001_f64;
        let split_origin = 147.89333333333337_f64;
        assert_eq!(
            device_x(whole_origin, position - whole_origin),
            device_x(split_origin, position - split_origin),
        );
    }
}
