struct ParticleEffectEngine {
    particles: Vec<Box<dyn Particle>>,
}

trait Particle {}

impl ParticleEffectEngine {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, particle: Box<dyn Particle>) {
        self.particles.push(particle);
    }
}
