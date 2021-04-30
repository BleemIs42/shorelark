struct Network {
  layers: Vec<Layer>,
}

struct LayerTopology {
  pub neurons: usize,
}

impl Network {
  pub fn new(layers: Vec<Layer>) -> Self {
      Self { layers }
  }
  pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
      assert!(layers.len() > 1);

      let layers = layers
          .windows(2)
          .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
          .collect();

      Self { layers }
  }
  pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
      // fold as js reduce fn
      self.layers
          .iter()
          .fold(inputs, |inputs, layer| layer.propagate(inputs))
  }
}

