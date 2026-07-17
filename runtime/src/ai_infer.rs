pub trait AiModel {
    fn infer(&mut self, input: &[f32], output: &mut [f32]);
}
pub struct TinyYolo {
    pub weights: &'static [f32],
}
impl AiModel for TinyYolo {
    fn infer(&mut self, _i: &[f32], o: &mut [f32]) {
        for v in o.iter_mut() {
            *v = 0.0;
        }
    }
}
// AI inference engine for autonomous decision making
