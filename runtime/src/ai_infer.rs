pub trait AiModel { fn infer(&mut self, input: &[f32], output: &mut [f32]); } pub struct OnnxBridge; impl AiModel for OnnxBridge { fn infer(&mut self, _: &[f32], _: &mut [f32]) {} }
